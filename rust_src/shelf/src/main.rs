use std::io::{BufRead, BufReader};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::{env, thread};

use ctrlc;

mod server;
use server::{start, RequestMessage};

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

fn main() {
    // Handle Ctrl+C for graceful shutdown
    ctrlc::set_handler(move || {
        SHUTDOWN.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Get port from command line arguments or default to 10080
    let args: Vec<String> = env::args().collect();
    let port = args
        .get(1)
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(9080);

    // We'll use a channel for receiving requests from the server
    let (request_tx, request_rx) = mpsc::channel::<RequestMessage>();

    // `on_request` closure: when the server receives a request, it sends it through `request_tx`.
    let on_request = move |req_msg| {
        request_tx.send(req_msg).unwrap();
    };
    let server_handle = start(port, on_request);

    // Another thread monitors stdin. On receiving a line "<id> <response>", call respond_with
    let server_handle_for_stdin = server_handle.clone(); // Move into a variable to keep it accessible
    thread::spawn(move || {
        let stdin = std::io::stdin();
        let reader = BufReader::new(stdin);
        for line in reader.lines() {
            if SHUTDOWN.load(Ordering::SeqCst) {
                break;
            }

            let line = match line {
                Ok(l) => l.trim().to_string(),
                Err(_) => break,
            };
            if line.is_empty() {
                continue;
            }

            let mut parts = line.splitn(2, ' ');
            let id_str = parts.next();
            let resp_body = parts.next();

            if let (Some(id_str), Some(body)) = (id_str, resp_body) {
                if let Ok(id) = id_str.parse::<u64>() {
                    server_handle_for_stdin.respond_with(id, body);
                }
            }
        }
    });

    // The main thread: continuously receives RequestMessage and prints them to stdout
    for req in request_rx {
        if SHUTDOWN.load(Ordering::SeqCst) {
            break;
        }
        println!(
            "Received Request: ID={}, Method={}, Path={}, Body={}",
            req.id, req.method, req.path, req.body
        );
    }

    // Gracefully shut down
    server_handle.shutdown();
    println!("Exiting gracefully.");
}
