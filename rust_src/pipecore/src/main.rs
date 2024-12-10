mod pipes;
mod errors;
mod io;

fn prepare_pipes() -> Result<(), errors::PipeError> {
    pipes::create_pipe("alice_in")?;
    pipes::create_pipe("alice_out")?;
    Ok(())
}

fn cleanup_pipes() -> Result<(), errors::PipeError> {
    pipes::remove_pipe("alice_in")?;
    pipes::remove_pipe("alice_out")?;
    Ok(())
}

// fn to_upper(input: &str) -> String {
//     input.to_uppercase()
// }

fn main() {
    match prepare_pipes() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
    match io::link("stdin", "alice_in") {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
    // match io::middleware("stdin", "stdout", to_upper) {
    //     Ok(_) => (),
    //     Err(e) => eprintln!("{}", e),
    // }
    match io::link("alice_out", "stdout") {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
    // detect and handle ctrl + C
    let (tx, rx) = std::sync::mpsc::channel();
    match ctrlc::set_handler(move || {
        tx.send(()).unwrap();
    }) {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
    loop {
        match rx.try_recv() {
            Ok(_) => break,
            Err(_) => (),
        }
    }
    match cleanup_pipes() {
        Ok(_) => (),
        Err(e) => eprintln!("{}", e),
    }
}