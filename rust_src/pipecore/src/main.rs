// 1. Create a named pipe with the name "alice_in" if it does not exist.
// 2. Create a named pipe with the name "alice_out" if it does not exist.
// 3. Monitor stdin for incoming messages. If any, proxy them to "alice_in" pipe.
// 4. Monitor the "alice_out" pipe for incoming messages. If any, proxy them to stdout.
use std::fs::remove_file;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::fs::FileTypeExt;
use std::path::Path;

extern crate libc;
use std::ffi::CString;


// create_pipe creates a named pipe with the given name if it does not exist. It returns a boolean.
// If the pipe is created or already exists, it returns true. Otherwise, it returns false.
fn create_pipe(name: &str) -> bool {
    let path = Path::new(name);
    if path.exists() {
        if let Ok(metadata) = path.metadata() { // got metadata
            if metadata.file_type().is_fifo() {
                return true; // pipe exists and is a fifo, return early
            } else {
                remove_file(path).unwrap(); // remove the file, will be created as a fifo later
            }
        } else {
            return false; // coule not get metadata, fail early
        }
    }
    let cstr = match CString::new(name) {
        Ok(cstr) => cstr,
        Err(_) => return false,
    };
    let mode = libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IWGRP | libc::S_IROTH | libc::S_IWOTH;
    let res = unsafe { libc::mkfifo(cstr.as_ptr(), mode) };
    res == 0 // return true if mkfifo succeeded
}

// remove_pipe removes the named pipe with the given name if it exists. It does not return anything.
fn remove_pipe(name: &str) {
    let path = Path::new(name);
    if path.exists() {
        remove_file(path).unwrap();
    }
}

// a function that reads from stdin and writes to the given pipe.
fn read_stdin_write_pipe(pipe: &str) {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut line = String::new();
    match stdin_lock.read_line(&mut line) {
        Ok(_) => {
            let path = Path::new(pipe);
            let file = std::fs::OpenOptions::new().write(true).open(path).unwrap();
            let mut writer = std::io::BufWriter::new(file);
            writer.write_all(line.as_bytes()).unwrap();
        }
        Err(_) => {}
    }
}

// a function that reads from the given pipe and writes to stdout.
fn read_pipe_write_stdout(pipe: &str) {
    let path = Path::new(pipe);
    let file = std::fs::OpenOptions::new().read(true).open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    match reader.read_line(&mut line) {
        Ok(_) => {
            let stdout = std::io::stdout();
            let mut stdout_lock = stdout.lock();
            stdout_lock.write_all(line.as_bytes()).unwrap();
        }
        Err(_) => {}
    }
}

fn main() {
    // try to create the pipes, if failed, remove the pipes and exit.
    if !create_pipe("alice_in") || !create_pipe("alice_out") {
        remove_pipe("alice_in");
        remove_pipe("alice_out");
        return;
    }
    loop {
        // read from stdin and write to "alice_in" pipe.
        read_stdin_write_pipe("alice_in");
        // read from "alice_out" pipe and write to stdout.
        read_pipe_write_stdout("alice_out");
    }
}