mod pipes;
mod errors;

fn main() {
    match pipes::create_pipe("alice_in") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("alice_in: {}", e);
            return;
        }
    }
    match pipes::create_pipe("alice_out") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("alice_out: {}", e);
            return;
        }
    }
    loop {
        let input = match pipes::read_stdin() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("stdin: {}", e);
                break;
            }
        };
        match pipes::write_pipe("alice_in", &input) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("alice_in: {}", e);
                break;
            }
        }
        let output = match pipes::read_pipe("alice_out") {
            Ok(s) => s,
            Err(e) => {
                eprintln!("alice_out: {}", e);
                break;
            }
        };
        match pipes::write_stdout(&output) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("stdout: {}", e);
                break;
            }
        }
    }
    match pipes::remove_pipe("alice_in") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("alice_in: {}", e);
        }
    }
    match pipes::remove_pipe("alice_out") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("alice_out: {}", e);
        }
    }
}