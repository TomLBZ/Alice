use std::net::TcpStream;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    mpsc::{self, Sender},
    Arc, Mutex,
};
use std::thread;

use tiny_http::{Method, Response, Server};

pub struct RequestMessage {
    pub id: u64,
    pub method: String,
    pub path: String,
    pub body: String,
}

/// Global shutdown flag for graceful termination
static SHUTDOWN: AtomicBool = AtomicBool::new(false);

pub fn shutdown() {
    SHUTDOWN.store(true, Ordering::SeqCst);
}

/// Starts the HTTP server on the given port.
///
/// # Arguments
/// - `port`: The port to listen on
/// - `on_receive`: A callback function/closure that is called when a request is received.
///                  It takes a `RequestMessage`.
/// - `respond_with`: A callback function/closure that is used to respond to a request.
///                    It takes `(id, response_body)` and sends the response back to the client.
///
/// The function returns a handle to the server thread. You can join on it if desired.
pub fn start<F, G>(port: u16, on_receive: F, respond_with: G) -> std::thread::JoinHandle<()>
where
    F: Fn(RequestMessage) + Send + 'static,
    G: Fn(u64, &str) + Send + 'static,
{
    let on_receive = Arc::new(on_receive);
    let respond_with = Arc::new(respond_with);

    thread::spawn(move || {
        let server_addr = format!("0.0.0.0:{}", port);
        let server = Server::http(&server_addr).expect("Failed to start HTTP server");
        println!("Server listening on http://{}", server_addr);

        let request_id_counter = Arc::new(AtomicU64::new(1));
        let pending_responses = Arc::new(Mutex::new(std::collections::HashMap::<u64, Sender<String>>::new()));

        // Create a clone of the map for the respond_with callback
        let pending_for_callback = Arc::clone(&pending_responses);

        // Wrap respond_with so that it can find the correct channel for responding.
        let respond_with_inner = {
            let pending_for_callback = Arc::clone(&pending_for_callback);
            move |id: u64, body: &str| {
                let mut map = pending_for_callback.lock().unwrap();
                if let Some(tx) = map.remove(&id) {
                    let _ = tx.send(body.to_string());
                } else {
                    eprintln!("No pending request with id {} to respond to", id);
                }
            }
        };

        // Now create a closure that calls the user-supplied respond_with callback
        // but we must ensure that the request has a waiting channel. The code above
        // sends via the stored map, so we just connect the given respond_with to this closure.
        let respond_with_user = Arc::clone(&respond_with);
        let respond_with = move |id: u64, body: &str| {
            // First respond using the internal logic
            respond_with_inner(id, body);
            // Optionally, call user callback after responding (if desired)
            respond_with_user(id, body);
        };

        // Main loop: accept connections until shutdown
        loop {
            if SHUTDOWN.load(Ordering::SeqCst) {
                break;
            }

            let request = match server.recv() {
                Ok(rq) => rq,
                Err(e) => {
                    if SHUTDOWN.load(Ordering::SeqCst) {
                        break;
                    }
                    eprintln!("Error receiving request: {}", e);
                    continue;
                }
            };

            if SHUTDOWN.load(Ordering::SeqCst) {
                break;
            }

            let id = request_id_counter.fetch_add(1, Ordering::SeqCst);
            let method = match request.method() {
                Method::Get => "GET".to_string(),
                Method::Post => "POST".to_string(),
                other => other.as_str().to_string(),
            };
            let path = request.url().to_string();

            let mut body_bytes = Vec::new();
            if let Some(mut reader) = request.as_reader() {
                let _ = std::io::Read::read_to_end(&mut reader, &mut body_bytes);
            }
            let body = String::from_utf8_lossy(&body_bytes).to_string();

            // Setup a channel to receive the response
            let (tx, rx) = mpsc::channel::<String>();
            {
                let mut map = pending_responses.lock().unwrap();
                map.insert(id, tx);
            }

            let on_receive_cloned = Arc::clone(&on_receive);

            // Spawn a thread to handle this request so multiple requests can be processed
            // concurrently. The on_receive callback is called here.
            thread::spawn(move || {
                // Call the on_receive callback
                let req_msg = RequestMessage { id, method, path, body };
                on_receive_cloned(req_msg);

                // Wait for the response
                let response_body = match rx.recv() {
                    Ok(resp) => resp,
                    Err(_) => "No response".to_string(),
                };

                let response = Response::from_string(response_body);
                if let Err(e) = request.respond(response) {
                    eprintln!("Failed to respond to request {}: {}", id, e);
                }
            });
        }

        println!("Server is shutting down gracefully.");
        // Try to unblock server if it's still waiting
        let _ = TcpStream::connect(&server_addr);
    })
}
