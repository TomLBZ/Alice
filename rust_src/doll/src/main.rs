use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::io::{self, BufRead, Write};
use std::fs::OpenOptions;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use ctrlc;

mod errors;
mod pipes;
use pipes::{create_pipe, remove_pipe};

struct ProcessInfo {
    filename: String,
    timestamp: u64,
    pid: u32,
    in_pipe: String,
    out_pipe: String,
    err_pipe: String,
    child: Child,
}

impl ProcessInfo {
    fn generate_pipes(filename: &str) -> (String, String, String, u64) {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let filename = filename.split('/').last().unwrap();
        let in_pipe = format!("{}_{}_i", filename, start);
        let out_pipe = format!("{}_{}_o", filename, start);
        let err_pipe = format!("{}_{}_e", filename, start);
        (in_pipe, out_pipe, err_pipe, start)
    }
}

/// Shared state: track running processes by filename.
struct AppState {
    processes: HashMap<String, ProcessInfo>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            processes: HashMap::new(),
        }
    }

    fn start_process(&mut self, filename: &str) -> io::Result<()> {
        println!("Starting process: {}", filename);
        if self.processes.contains_key(filename) {
            eprintln!("Process {} already running. Use 'down' or 'restart' if needed.", filename);
            return Ok(());
        }

        let (in_pipe, out_pipe, err_pipe, timestamp) = ProcessInfo::generate_pipes(filename);
        println!("Pipes: in={}, out={}, err={}", in_pipe, out_pipe, err_pipe);

        // Create pipes
        match create_pipe(&in_pipe) {
            Ok(_) => (),
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        }
        match create_pipe(&out_pipe) {
            Ok(_) => (),
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        }
        match create_pipe(&err_pipe) {
            Ok(_) => (),
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        }

        println!("Pipes created successfully.");
        // Spawn the process with redirected stdin, stdout, stderr
        let in_handle = OpenOptions::new().read(true).open(&in_pipe).unwrap();
        println!("in handle opened successfully.");
        let out_handle = OpenOptions::new().write(true).open(&out_pipe).unwrap();
        println!("out handle opened successfully.");
        let err_handle = OpenOptions::new().write(true).open(&err_pipe).unwrap();
        println!("err handle opened successfully.");
        match Command::new(filename)
            .stdin(Stdio::from(in_handle))
            .stdout(Stdio::from(out_handle))
            .stderr(Stdio::from(err_handle))
            .spawn() {
            Ok(child) => {
                println!("Process spawned successfully.");
                let pid = child.id();

                let info = ProcessInfo {
                    filename: filename.to_string(),
                    timestamp,
                    pid,
                    in_pipe,
                    out_pipe,
                    err_pipe,
                    child,
                };

                self.processes.insert(filename.to_string(), info);
                println!("Started {} with pid {}", filename, pid);
            },
            Err(e) => {
                println!("Failed to start process: {}", e);
                return Err(e);
            },
        }
        println!("Process started successfully.");

        Ok(())
    }

    fn restart_process(&mut self, filename: &str) -> io::Result<()> {
        self.stop_process(filename)?;
        self.start_process(filename)?;
        Ok(())
    }

    fn stop_process(&mut self, filename: &str) -> io::Result<()> {
        if let Some(mut info) = self.processes.remove(filename) {
            let _ = info.child.kill(); // kill the process if still running
            let _ = info.child.wait();
            // Remove pipes
            let _ = remove_pipe(&info.in_pipe);
            let _ = remove_pipe(&info.out_pipe);
            let _ = remove_pipe(&info.err_pipe);

            println!("Stopped {} and removed pipes", filename);
        } else {
            eprintln!("No such running process: {}", filename);
        }
        Ok(())
    }

    fn send_input(&self, filename: &str, text: &str) -> io::Result<()> {
        if let Some(info) = self.processes.get(filename) {
            // The in_pipe is read by the process, we must write to it
            let mut f = OpenOptions::new().write(true).open(&info.in_pipe)?;
            writeln!(f, "{}", text)?;
        } else {
            eprintln!("No such running process: {}", filename);
        }
        Ok(())
    }

    fn info(&self, filename: Option<&str>) {
        if let Some(fname) = filename {
            if let Some(info) = self.processes.get(fname) {
                println!("{}: pid={}, timestamp={}, in={}, out={}, err={}", 
                         info.filename, info.pid, info.timestamp, info.in_pipe, info.out_pipe, info.err_pipe);
            } else {
                eprintln!("No such process: {}", fname);
            }
        } else {
            for (fname, info) in &self.processes {
                println!("{}: pid={}, timestamp={}, in={}, out={}, err={}", 
                         fname, info.pid, info.timestamp, info.in_pipe, info.out_pipe, info.err_pipe);
            }
        }
    }

    fn monitor_and_restart(&mut self) -> io::Result<()> {
        // This function checks if any process has exited and restarts it
        // Ideally, this could be run periodically or triggered by user input.
        // One approach: after receiving every command, we try to reap dead children.
        // If a child is dead, we restart it as required by the rules.
        let keys: Vec<String> = self.processes.keys().cloned().collect();
        for k in keys {
            let restart_needed: bool;
            if let Some(info) = self.processes.get_mut(&k) {
                if let Ok(Some(status)) = info.child.try_wait() {
                    eprintln!("Process {} exited with status: {:?}", k, status);
                    restart_needed = true;
                } else {
                    restart_needed = false;
                }
            } else {
                restart_needed = false;
            }

            if restart_needed {
                // remove old info from map and restart
                let _ = self.stop_process(&k);
                let _ = self.start_process(&k);
            }
        }
        Ok(())
    }

    fn shutdown_all(&mut self) {
        // On ctrl+C or exit
        let keys: Vec<String> = self.processes.keys().cloned().collect();
        for k in keys {
            let _ = self.stop_process(&k);
        }
    }
}

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
        let mut parts = line.split_whitespace();
        let command = parts.next();

        {
            let mut s = state.lock().unwrap();
            // Before processing command, check if any processes have crashed and need restart
            let _ = s.monitor_and_restart();
        }

        match command {
            Some("up") => {
                let filename = parts.next();
                if let Some(f) = filename {
                    let mut s = state.lock().unwrap();
                    if let Err(e) = s.start_process(f) {
                        eprintln!("Failed to start {}: {}", f, e);
                    }
                } else {
                    eprintln!("Usage: up <filename>");
                }
            }
            Some("down") => {
                let filename = parts.next();
                if let Some(f) = filename {
                    let mut s = state.lock().unwrap();
                    if let Err(e) = s.stop_process(f) {
                        eprintln!("Failed to stop {}: {}", f, e);
                    }
                } else {
                    eprintln!("Usage: down <filename>");
                }
            }
            Some("info") => {
                let filename = parts.next();
                let s = state.lock().unwrap();
                s.info(filename);
            }
            Some("send") => {
                let filename = parts.next();
                if let Some(f) = filename {
                    let rest: String = parts.collect::<Vec<_>>().join(" ");
                    let s = state.lock().unwrap();
                    if let Err(e) = s.send_input(f, &rest) {
                        eprintln!("Failed to send to {}: {}", f, e);
                    }
                } else {
                    eprintln!("Usage: send <filename> <TEXT>");
                }
            }
            Some("restart") => {
                let filename = parts.next();
                if let Some(f) = filename {
                    let mut s = state.lock().unwrap();
                    if let Err(e) = s.restart_process(f) {
                        eprintln!("Failed to restart {}: {}", f, e);
                    }
                } else {
                    eprintln!("Usage: restart <filename>");
                }
            }
            Some(cmd) if cmd.is_empty() => {
                // Ignore empty lines
            }
            Some(_) => {
                eprintln!("Unknown command.");
            }
            None => {}
        }
    }

    Ok(())
}
