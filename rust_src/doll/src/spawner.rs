use std::fmt::{self, Display, Formatter};
use std::process::{Child, Command, Stdio};
use std::io::{self, Error, ErrorKind, Read, Result, Write};
use std::time::SystemTime;
use std::path::Path;
use std::os::unix::fs::FileTypeExt; // for checking if a file is a FIFO
use std::os::fd::{AsRawFd, FromRawFd};
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
    fn receive(&mut self) -> Result<String>;
    fn receive_err(&mut self) -> Result<String>;
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
        let mut f = OpenOptions::new().write(true).open(&self.in_pipe)?;
        writeln!(f, "{}", text)?;
        Ok(())
    }
    fn receive(&mut self) -> Result<String> {
        let mut f = OpenOptions::new().read(true).open(&self.out_pipe)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    fn receive_err(&mut self) -> Result<String> {
        let mut f = OpenOptions::new().read(true).open(&self.err_pipe)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
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

fn link_by_fd(ifd: i32, ofd: i32) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut reader = Box::new(unsafe { std::fs::File::from_raw_fd(ifd) });
            let mut writer = Box::new(unsafe { std::fs::File::from_raw_fd(ofd) });
            let result = io::copy(&mut reader, &mut writer);
            match result {
                Ok(0) => (),
                Ok(_) => (),
                Err(e) => eprintln!("Failed to copy data: {}", e),
            }
        }
    });
    Ok(())
}

pub fn spawn(exec_name: &str) -> Result<ProcessInfo> {
    let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_secs(),
        Err(e) => return Err(build_io_error(&format!("Failed to get timestamp: {}", e))),
    };
    let in_pipe = format!("{}{}.i", exec_name, timestamp);
    let out_pipe = format!("{}{}.o", exec_name, timestamp);
    let err_pipe = format!("{}{}.e", exec_name, timestamp);
    create_pipe(&in_pipe)?;
    create_pipe(&out_pipe)?;
    create_pipe(&err_pipe)?;
    println!("Pipes created successfully.");
    // Spawn the process with redirected stdin, stdout, stderr (anonymous pipes)
    let mut child = Command::new(exec_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    println!("Process spawned successfully.");
    let pid = child.id();
    let child_in = child.stdin.take().expect("Failed to open stdin.");
    let child_out = child.stdout.take().expect("Failed to open stdout.");
    let child_err = child.stderr.take().expect("Failed to open stderr.");
    let child_in_fd = child_in.as_raw_fd();
    let child_out_fd = child_out.as_raw_fd();
    let child_err_fd = child_err.as_raw_fd();
    let in_pipe_cstr = CString::new(in_pipe.clone()).unwrap();
    let out_pipe_cstr = CString::new(out_pipe.clone()).unwrap();
    let err_pipe_cstr = CString::new(err_pipe.clone()).unwrap();
    let in_pipe_fd = unsafe { libc::open(in_pipe_cstr.as_ptr(), libc::O_WRONLY) };
    let out_pipe_fd = unsafe { libc::open(out_pipe_cstr.as_ptr(), libc::O_RDONLY) };
    let err_pipe_fd = unsafe { libc::open(err_pipe_cstr.as_ptr(), libc::O_RDONLY) };
    link_by_fd(in_pipe_fd, child_in_fd)?;
    link_by_fd(child_out_fd, out_pipe_fd)?;
    link_by_fd(child_err_fd, err_pipe_fd)?;

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