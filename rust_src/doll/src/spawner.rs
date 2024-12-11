use std::fmt::{self, Display, Formatter};
use std::process::{Child, Command, Stdio};
use std::io::{BufRead, BufReader, Error, ErrorKind, Result, Write};
use std::time::SystemTime;
use std::path::Path;
use std::os::unix::fs::FileTypeExt; // for checking if a file is a FIFO
use std::ffi::CString;
use std::fs::{remove_file, OpenOptions};
use std::thread;

pub struct ProcessInfo {
    pub exec_name: String,
    pub timestamp: u64,
    pub pid: u32,
    pub in_pipe: String,
    pub out_pipe: String,
    pub err_pipe: String,
    pub child: Child,
}

pub trait Control {
    fn stop(&mut self) -> Result<()>;
    fn restart(&mut self) -> Result<()>;
    fn auto_restart(&mut self) -> Result<()>;
    fn send(&mut self, text: &str) -> Result<()>;
    fn receive(&mut self, error: bool) -> Result<String>;
}

impl Control for ProcessInfo {
    fn stop(&mut self) -> Result<()> {
        self.child.kill()?;
        self.child.wait()?;
        remove_pipe(&self.in_pipe)?;
        remove_pipe(&self.out_pipe)?;
        remove_pipe(&self.err_pipe)?;
        Ok(())
    }
    fn restart(&mut self) -> Result<()> {
        self.stop()?;
        let tmp_info = spawn(&self.exec_name)?;
        self.timestamp = tmp_info.timestamp;
        self.pid = tmp_info.pid;
        self.in_pipe = tmp_info.in_pipe;
        self.out_pipe = tmp_info.out_pipe;
        self.err_pipe = tmp_info.err_pipe;
        self.child = tmp_info.child;
        Ok(())
    }
    fn auto_restart(&mut self) -> Result<()> {
        let status = self.child.try_wait()?;
        if status.is_none() {
            return Ok(());
        }
        self.restart()?;
        Ok(())
    }
    fn send(&mut self, text: &str) -> Result<()> {
        let mut f = OpenOptions::new().read(true).write(true).open(&self.in_pipe)?;
        writeln!(f, "{}", text)?;
        println!("Sent to {}: {}", self.in_pipe, text);
        Ok(())
    }
    fn receive(&mut self, error: bool) -> Result<String> {
        let path = if error { &self.err_pipe } else { &self.out_pipe };
        let f = OpenOptions::new().read(true).write(true).open(path)?;
        println!("Reading from {}", self.out_pipe);
        let mut reader = BufReader::new(&f);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        Ok(line)
    }
}

impl Display for ProcessInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ProcessInfo: exec_name={}, timestamp={}, pid={}, in_pipe={}, out_pipe={}, err_pipe={}",
               self.exec_name, self.timestamp, self.pid, self.in_pipe, self.out_pipe, self.err_pipe)
    }
}

fn build_io_error(e: &str) -> Error {
    Error::new(ErrorKind::Other, e)
}

fn fifo_pipe_available(pipe: &str) -> Result<bool> {
    let path = Path::new(pipe);
    if path.exists() {
        match path.metadata() {
            Ok(metadata) => {
                if metadata.file_type().is_fifo() {
                    return Ok(true);
                } else {
                    return Err(build_io_error("Pipe is not a FIFO."));
                }
            }
            Err(_) => return Err(build_io_error("Failed to get metadata for pipe.")),
        }
    }
    Ok(false)
}

fn create_pipe(name: &str) -> Result<()> {
    let fifo_available = match fifo_pipe_available(name) {
        Ok(b) => b,
        Err(e) => return Err(e),
    };
    if fifo_available {
        return Ok(());
    }
    let c_name = match CString::new(name) {
        Ok(s) => s,
        Err(_) => return Err(build_io_error("Invalid pipe name.")),
    };

    let mode = libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IWGRP | libc::S_IROTH | libc::S_IWOTH;
    let res = unsafe { libc::mkfifo(c_name.as_ptr(), mode) };
    if res == 0 {
        Ok(())
    } else {
        let errno = unsafe { *libc::__errno_location() };
        if errno == libc::EEXIST {
            Ok(())
        } else {
            Err(build_io_error("Failed to create pipe."))
        }
    }
}

fn remove_pipe(name: &str) -> Result<()> {
    match remove_file(name) {
        Ok(_) => Ok(()),
        Err(_) => Err(build_io_error("Failed to remove pipe.")),
    }
}

fn forward_io<R, W>(mut reader: R, mut writer: W) where R: BufRead + Send + 'static, W: Write + Send + 'static {
    thread::spawn(move || {
        loop { // never breaks
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => continue, // EOF on reader
                Ok(_) => { // read a line
                    if let Err(_) = writer.write_all(line.as_bytes()) {
                        continue; // Writer failed to write text
                    }
                }
                Err(_) => continue, // Error reading
            }
        }
    });
}

pub fn spawn(exec_name: &str) -> Result<ProcessInfo> {
    let mut child = Command::new("stdbuf")
        .arg("-oL") // output line buffered
        .arg("-eL") // error line buffered
        .arg(exec_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let pid = child.id();
    let child_in = child.stdin.take().expect("Failed to open stdin.");
    let child_out = child.stdout.take().expect("Failed to open stdout.");
    let child_err = child.stderr.take().expect("Failed to open stderr.");
    println!("Child process spawned.");
    let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_secs(),
        Err(e) => return Err(build_io_error(&format!("Failed to get timestamp: {}", e))),
    };
    // exec_name may contain path. needs to get only the executable name
    let exec_fname = Path::new(exec_name).file_name().unwrap().to_str().unwrap();
    let in_pipe = format!("{}{}.i", exec_fname, timestamp);
    let out_pipe = format!("{}{}.o", exec_fname, timestamp);
    let err_pipe = format!("{}{}.e", exec_fname, timestamp);
    create_pipe(&in_pipe)?;
    create_pipe(&out_pipe)?;
    create_pipe(&err_pipe)?;
    println!("Pipes created successfully.");
    let in_pipe_file = OpenOptions::new().read(true).write(true).open(&in_pipe)?;
    let out_pipe_file = OpenOptions::new().read(true).write(true).open(&out_pipe)?;
    let err_pipe_file = OpenOptions::new().read(true).write(true).open(&err_pipe)?;
    forward_io(BufReader::new(in_pipe_file), child_in);
    forward_io(BufReader::new(child_out), out_pipe_file);
    forward_io(BufReader::new(child_err), err_pipe_file);
    println!("Forwarded IO.");

    let info = ProcessInfo {
        exec_name: exec_name.to_string(),
        timestamp,
        pid,
        in_pipe,
        out_pipe,
        err_pipe,
        child,
    };
    println!("Started {} with pid {}", exec_name, pid);
    Ok(info)
}