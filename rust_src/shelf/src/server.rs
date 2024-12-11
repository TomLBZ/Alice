use std::thread;
use std::fmt::Display;
use std:: sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Sender},
    Arc, Mutex,
};

use tiny_http::{Method, Response, Server};

pub struct RequestMessage {
    pub id: u64,
    pub method: String,
    pub path: String,
    pub body: String,
}

impl Display for RequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]{}:{} {{{}}}",
            self.id, self.method, self.path, self.body
        )
    }
}

#[derive(Clone)]
pub struct ServerHandle {
    respond_with_fn: Sender<(u64, String)>,
    shutdown_flag: Arc<AtomicBool>,
}

impl ServerHandle {
    /// This function can be called to respond to a pending request.
    /// `id` is the request ID, `resp` is the response body.
    pub fn respond_with(&self, id: u64, resp: &str) {
        if !self.shutdown_flag.load(Ordering::SeqCst) {
            let _ = self.respond_with_fn.send((id, resp.to_string()));
        }
    }

    /// Set the shutdown flag, indicating the server should stop.
    pub fn shutdown(&self) {
        self.shutdown_flag.store(true, Ordering::SeqCst);
    }
}

/// Starts the server on the given `port`.
/// `on_request` is a closure called whenever a request arrives.
///
/// Returns a `ServerHandle` that can be used to respond to requests.
pub fn start<F>(port: u16, on_request: F) -> ServerHandle
where
    F: Fn(RequestMessage) + Send + Sync + 'static,
{
    let server_addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&server_addr).expect("Failed to start HTTP server");
    println!("Server listening on http://{}", server_addr);

    // Shutdown flag
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    // Channel to receive responses that will be sent to clients
    let (resp_tx, resp_rx) = mpsc::channel::<(u64, String)>();
    // Map of request ID to the actual `tiny_http::Request` object
    let pending_requests =
        Arc::new(Mutex::new(std::collections::HashMap::<u64, tiny_http::Request>::new()));
    let on_request = Arc::new(on_request);
    
    let pending_requests_server = Arc::clone(&pending_requests);
    let shutdown_server = Arc::clone(&shutdown_flag);

    // Spawn server thread to accept requests
    thread::spawn(move || {
        let mut next_id: u64 = 1;
        loop {
            if shutdown_server.load(Ordering::SeqCst) {
                break;
            }
            let mut rq = match server.recv() {
                Ok(r) => r,
                Err(_) => {
                    if shutdown_server.load(Ordering::SeqCst) {
                        break;
                    }
                    continue;
                }
            };

            let id = next_id;
            next_id += 1;

            let method = match rq.method() {
                Method::Get => "GET".to_string(),
                Method::Post => "POST".to_string(),
                other => other.as_str().to_string(),
            };
            let path = rq.url().to_string();
            let mut body_bytes = Vec::new();
            let _ = rq.as_reader().read_to_end(&mut body_bytes);
            let body = String::from_utf8_lossy(&body_bytes).to_string();

            {
                let mut map = pending_requests_server.lock().unwrap();
                map.insert(id, rq);
            }

            let req_msg = RequestMessage {
                id,
                method,
                path,
                body,
            };

            // Call the callback on the main thread side
            on_request(req_msg);
        }
        // Exiting server loop
    });

    let pending_requests_resp = Arc::clone(&pending_requests);
    let shutdown_resp = Arc::clone(&shutdown_flag);

    // Spawn response thread: waits for responses from main and responds to requests
    thread::spawn(move || {
        while !shutdown_resp.load(Ordering::SeqCst) {
            let (id, resp_str) = match resp_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            let mut map = pending_requests_resp.lock().unwrap();
            if let Some(rq) = map.remove(&id) {
                let response = Response::from_string(resp_str);
                let _ = rq.respond(response);
            }
        }
    });

    ServerHandle {
        respond_with_fn: resp_tx,
        shutdown_flag,
    }
}
