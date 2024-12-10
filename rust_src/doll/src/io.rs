use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::thread;

fn get_reader(iname: &str) -> Result<Box<dyn Read + Send>, io::Error> {
    if iname == "stdin" {
        Ok(Box::new(io::stdin()))
    } else {
        Ok(Box::new(OpenOptions::new().read(true).open(iname)?))
    }
}

fn get_writer(oname: &str) -> Result<Box<dyn Write + Send>, io::Error> {
    if oname == "stdout" {
        Ok(Box::new(io::stdout()))
    } else {
        Ok(Box::new(OpenOptions::new().write(true).open(oname)?))
    }
}

pub fn link(iname: &str, oname: &str) -> Result<(), io::Error> {
    let i_name = iname.to_string();
    let o_name = oname.to_string();
    // Spawn a background thread to handle the actual IO forwarding
    thread::spawn(move || {
        loop {
            let mut reader = match get_reader(&i_name) {
                Ok(reader) => reader,
                Err(e) => {
                    eprintln!("Failed to open input file: {}", e);
                    continue;
                }
            };
            let mut writer = match get_writer(&o_name) {
                Ok(writer) => writer,
                Err(e) => {
                    eprintln!("Failed to open output file: {}", e);
                    continue;
                }
            };
            let result = io::copy(&mut reader, &mut writer);
            match result {
                Ok(0) => { continue; }
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Failed to copy data: {}", e);
                    continue;
                }
            }
        }
    });

    Ok(())
}

pub fn middleware<F>(iname: &str, oname: &str, f: F) -> Result<(), io::Error>
    where F: Fn(&str) -> String + Send + 'static,
{
    let i_name = iname.to_string();
    let o_name = oname.to_string();
    // Spawn a background thread to handle reading, transforming, and writing
    thread::spawn(move || {
        loop {
            let reader = match get_reader(&i_name) {
                Ok(reader) => reader,
                Err(e) => {
                    eprintln!("Failed to open input file: {}", e);
                    continue;
                }
            };
            let mut writer = match get_writer(&o_name) {
                Ok(writer) => writer,
                Err(e) => {
                    eprintln!("Failed to open output file: {}", e);
                    continue;
                }
            };
            let buffered = BufReader::new(reader);
            for line in buffered.lines() {
                match line {
                    Ok(input_line) => {
                        let output_line = f(&input_line);
                        if let Err(e) = writeln!(writer, "{}", output_line) {
                            eprintln!("Failed to write transformed data: {}", e);
                            continue;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read line: {}", e);
                        continue;
                    }
                }
            }
        }
    });
    Ok(())
}