use ctrlc;
use std::env;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    mpsc::{self, Sender},
    Arc, Mutex,
};
use std::thread;
use tiny_http::{Method, Response, Server};


/// A request message sent from the web thread to the output (stdout) thread.
struct RequestMessage {
    id: u64,
    method: String,
    path: String,
    body: String,
}

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

fn main() {
    // Handle Ctrl+C to trigger a graceful shutdown
    ctrlc::set_handler(move || {
        SHUTDOWN.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Get port from command line arguments or default to 9080
    let args: Vec<String> = env::args().collect();
    let port = args
        .get(1)
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(9080);

    let server_addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&server_addr).expect("Failed to start HTTP server");
    println!("Listening on http://{}", server_addr);

    // Channels:
    // Web -> Output (requests to print)
    let (web_to_output_tx, web_to_output_rx) = mpsc::channel::<RequestMessage>();

    // A shared map from request ID to a sender waiting for a response.
    // The input thread will read from stdin and send responses here.
    let pending_responses =
        Arc::new(Mutex::new(std::collections::HashMap::<u64, Sender<String>>::new()));

    // Start the output thread (just prints requests to stdout)
    {
        let web_to_output_rx = web_to_output_rx;
        thread::spawn(move || {
            for req_msg in web_to_output_rx {
                println!(
                    "REQUEST {} {} {} {}\n",
                    req_msg.id, req_msg.method, req_msg.path, req_msg.body
                );
            }
        });
    }

    // Start the input thread:
    // This thread reads from stdin lines in the format:
    // "<id> <response>"
    // If <id> matches a pending request, send the response back to that request thread.
    {
        let pending_responses = Arc::clone(&pending_responses);
        thread::spawn(move || {
            let stdin = std::io::stdin();
            let reader = BufReader::new(stdin);

            for line_result in reader.lines() {
                if SHUTDOWN.load(Ordering::SeqCst) {
                    break;
                }
                let line = match line_result {
                    Ok(l) => l.trim().to_string(),
                    Err(_) => break,
                };

                if line.is_empty() {
                    continue;
                }

                let mut parts = line.splitn(2, ' ');
                let id_part = parts.next();
                let body_part = parts.next();

                let (id, body) = match (id_part, body_part) {
                    (Some(id_str), Some(body_str)) => {
                        if let Ok(id) = id_str.parse::<u64>() {
                            (id, body_str.to_string())
                        } else {
                            continue; // Invalid format
                        }
                    }
                    _ => continue,
                };

                let mut map = pending_responses.lock().unwrap();
                if let Some(sender) = map.remove(&id) {
                    let _ = sender.send(body);
                } // else just ignore if ID not found
            }
        });
    }

    let request_id_counter = Arc::new(AtomicU64::new(1));
    let pending_responses_main = Arc::clone(&pending_responses);

    // Main loop: accept connections and spawn a thread to handle each request
    loop {
        if SHUTDOWN.load(Ordering::SeqCst) {
            break;
        }

        // Use a non-blocking check with a trick:
        // tiny_http::Server doesn't provide a direct shutdown method, 
        // so if we are shutting down, we send a dummy request to ourselves to unblock.
        match server.recv() {
            Ok(mut request) => {
                if SHUTDOWN.load(Ordering::SeqCst) {
                    // If shutdown requested after receiving a request, we can still handle it or drop it.
                    // Let's handle gracefully: spawn a thread as we do.
                }

                let id = request_id_counter.fetch_add(1, Ordering::SeqCst);
                let method = match request.method() {
                    Method::Get => "GET".to_string(),
                    Method::Post => "POST".to_string(),
                    other => other.as_str().to_string(),
                };
                let path = request.url().to_string();

                let mut body_bytes = Vec::new();
                let mut reader = request.as_reader();
                let _ = std::io::Read::read_to_end(&mut reader, &mut body_bytes);
                let body = String::from_utf8_lossy(&body_bytes).to_string();

                let web_to_output_tx_clone = web_to_output_tx.clone();
                let pending_responses_clone = Arc::clone(&pending_responses_main);

                // Spawn a thread to handle this request
                thread::spawn(move || {
                    // Send to output thread
                    web_to_output_tx_clone
                        .send(RequestMessage {
                            id,
                            method: method.clone(),
                            path: path.clone(),
                            body: body.clone(),
                        })
                        .expect("Failed to send request message to output thread");

                    // Prepare a channel to receive the response
                    let (tx, rx) = mpsc::channel::<String>();
                    {
                        let mut map = pending_responses_clone.lock().unwrap();
                        map.insert(id, tx);
                    }

                    // Wait for the response from input thread
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
            Err(e) => {
                if SHUTDOWN.load(Ordering::SeqCst) {
                    // Likely we forced an unblock request
                    break;
                }
                eprintln!("Error receiving request: {}", e);
                // It's possible that server recv failed due to some error, we continue or break based on need
            }
        }
    }

    // Graceful shutdown steps:
    // If we got here due to Ctrl+C (SHUTDOWN = true), let's try to unblock the server if it's still blocking.
    // A known trick: Make a dummy request to let server.recv() return.
    if !SHUTDOWN.load(Ordering::SeqCst) {
        // If we're here because of some other reason, let's just exit.
        return;
    }

    // If the server is still possibly blocking, try to force a dummy request:
    let _ = TcpStream::connect(&server_addr);

    println!("Server is shutting down gracefully.");
}
