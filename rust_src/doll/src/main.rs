use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use ctrlc;

mod spawner;
mod parser;
mod state;
use parser::{Command, parse_command};
use state::{AppState, Manage};

fn main() -> io::Result<()> {
    let state = Arc::new(Mutex::new(AppState::new()));
    {
        let state = Arc::clone(&state);
        ctrlc::set_handler(move || {
            // On ctrl+C
            let mut s = state.lock().unwrap();
            s.shutdown_all();
            std::process::exit(0);
        }).expect("Error setting Ctrl-C handler");
    }
    println!("doll is running. Commands: up, down, send, info, restart, ctrl+C to exit.");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let command_info = match parse_command(&line) {
            Some(info) => info,
            None => continue,
        };
        {
            let mut s = state.lock().unwrap();
            // Before processing command, check if any processes have crashed and need restart
            let _ = s.monitor_and_restart();
        }
        match command_info.cmd {
            Command::Up => {
                let mut s = state.lock().unwrap();
                if let Err(e) = s.start_process(&command_info.exec_name) {
                    eprintln!("Failed to start {}: {}", command_info.exec_name, e);
                }
            }
            Command::Down => {
                let mut s = state.lock().unwrap();
                if let Err(e) = s.stop_process(&command_info.exec_name) {
                    eprintln!("Failed to stop {}: {}", command_info.exec_name, e);
                }
            }
            Command::Info => {
                let s = state.lock().unwrap();
                if command_info.exec_name.is_empty() {
                    s.get_info(None);
                } else {
                    s.get_info(Some(&command_info.exec_name));
                }
            }
            Command::Send => {
                let mut s = state.lock().unwrap();
                if let Err(e) = s.send_input(&command_info.exec_name, &command_info.args.join(" ")) {
                    eprintln!("Failed to send to {}: {}", command_info.exec_name, e);
                }
            }
            Command::Receive => {
                let mut s = state.lock().unwrap();
                match s.get_output(&command_info.exec_name) {
                    Ok(output) => println!("{}", output),
                    Err(e) => eprintln!("Failed to receive from {}: {}", command_info.exec_name, e),
                }
            }
            Command::ReceiveErr => {
                let mut s = state.lock().unwrap();
                match s.get_error(&command_info.exec_name) {
                    Ok(output) => println!("{}", output),
                    Err(e) => eprintln!("Failed to receive error from {}: {}", command_info.exec_name, e),
                }
            }
            Command::Restart => {
                let mut s = state.lock().unwrap();
                if let Err(e) = s.restart_process(&command_info.exec_name) {
                    eprintln!("Failed to restart {}: {}", command_info.exec_name, e);
                }
            }
            Command::Unknown => {
                eprintln!("Unknown command.");
            }
        }
    }

    Ok(())
}
