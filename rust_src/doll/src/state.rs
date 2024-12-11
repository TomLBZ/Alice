use std::collections::HashMap;
use std::io::{Result, Error, ErrorKind};
use crate::spawner::{spawn, Control, ProcessInfo};

pub struct AppState {
    processes: HashMap<String, ProcessInfo>,
}

pub trait Manage {
    fn start_process(&mut self, filename: &str) -> Result<()>;
    fn restart_process(&mut self, filename: &str) -> Result<()>;
    fn stop_process(&mut self, filename: &str) -> Result<()>;
    fn send_input(&mut self, filename: &str, text: &str) -> Result<()>;
    fn get_output(&mut self, filename: &str) -> Result<String>;
    fn get_error(&mut self, filename: &str) -> Result<String>;
    fn get_info(&self, filename: Option<&str>);
    fn monitor_and_restart(&mut self) -> Result<()>;
    fn shutdown_all(&mut self);
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            processes: HashMap::new(),
        }
    }
}

impl Manage for AppState {
    fn start_process(&mut self, filename: &str) -> Result<()> {
        let info = spawn(filename)?;
        self.processes.insert(filename.to_string(), info);
        Ok(())
    }

    fn restart_process(&mut self, filename: &str) -> Result<()> {
        self.stop_process(filename)?;
        self.start_process(filename)?;
        Ok(())
    }

    fn stop_process(&mut self, filename: &str) -> Result<()> {
        if let Some(mut info) = self.processes.remove(filename) {
            match info.stop() {
                Ok(_) => println!("Stopped {}", filename),
                Err(e) => eprintln!("Failed to stop {}: {}", filename, e),
            }
        } else {
            eprintln!("No such running process: {}", filename);
        }
        Ok(())
    }

    fn send_input(&mut self, filename: &str, text: &str) -> Result<()> {
        if let Some(info) = self.processes.get_mut(filename) {
            info.send(text)?;
        } else {
            eprintln!("No such running process: {}", filename);
        }
        Ok(())
    }

    fn get_output(&mut self, filename: &str) -> Result<String> {
        if let Some(info) = self.processes.get_mut(filename) {
            info.receive(false)
        } else {
            eprintln!("No such running process: {}", filename);
            Err(Error::new(ErrorKind::NotFound, "Process not found"))
        }
    }

    fn get_error(&mut self, filename: &str) -> Result<String> {
        if let Some(info) = self.processes.get_mut(filename) {
            info.receive(true)
        } else {
            eprintln!("No such running process: {}", filename);
            Err(Error::new(ErrorKind::NotFound, "Process not found"))
        }
    }

    fn get_info(&self, filename: Option<&str>) {
        if let Some(fname) = filename {
            if let Some(info) = self.processes.get(fname) {
                println!("{}", info);
            } else {
                eprintln!("No such process: {}", fname);
            }
        } else {
            for (k, v) in &self.processes {
                println!("{}: {}", k, v);
            }
        }
    }

    fn monitor_and_restart(&mut self) -> Result<()> {
        // Ideally, this could be run periodically or triggered by user input.
        let keys: Vec<String> = self.processes.keys().cloned().collect();
        for k in keys {
            if let Some(info) = self.processes.get_mut(&k) {
                if let Err(e) = info.auto_restart() {
                    eprintln!("Failed to restart {}: {}", k, e);
                }
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
