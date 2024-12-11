pub struct CommandInfo {
    pub cmd: Command,
    pub exec_name: String,
    pub args: Vec<String>,
}

pub enum Command {
    Up,
    Down,
    Info,
    Send,
    Receive,
    ReceiveErr,
    Restart,
    Unknown,
}

impl Command {
    fn check_validity(&self, exec_name: &str, args: &[String]) -> bool {
        match self {
            Command::Up => args.is_empty() && !exec_name.is_empty(), // up exec_name
            Command::Down => args.is_empty() && !exec_name.is_empty(), // down exec_name
            Command::Info => args.is_empty(), // info OR info exec_name
            Command::Send => !args.is_empty() && !exec_name.is_empty(), // send exec_name text
            Command::Receive => args.is_empty() && !exec_name.is_empty(), // receive exec_name
            Command::ReceiveErr => args.is_empty() && !exec_name.is_empty(), // error exec_name
            Command::Restart => args.is_empty() && !exec_name.is_empty(), // restart exec_name
            Command::Unknown => false,
        }
    }
    fn get_usage(&self) -> String {
        match self {
            Command::Up => "Usage: up exec_name".to_string(),
            Command::Down => "Usage: down exec_name".to_string(),
            Command::Info => "Usage: info [exec_name]".to_string(),
            Command::Send => "Usage: send exec_name text".to_string(),
            Command::Receive => "Usage: receive exec_name".to_string(),
            Command::ReceiveErr => "Usage: error exec_name".to_string(),
            Command::Restart => "Usage: restart exec_name".to_string(),
            Command::Unknown => "Error: unknown command".to_string(),
        }
    }
}

pub fn parse_command(line: &str) -> Option<CommandInfo> {
    let mut parts = line.split_whitespace();
    let cmd = parts.next().unwrap_or_default().to_string();
    let cmd = cmd.to_lowercase();
    let command = match cmd.as_str() {
        "up" => Command::Up,
        "down" => Command::Down,
        "info" => Command::Info,
        "send" => Command::Send,
        "receive" => Command::Receive,
        "error" => Command::ReceiveErr,
        "restart" => Command::Restart,
        _ => Command::Unknown,
    };
    let exec_name = parts.next().unwrap_or_default().to_string();
    let args: Vec<String> = parts.map(|s| s.to_string()).collect();
    let validity = command.check_validity(&exec_name, &args);
    if validity {
        Some(CommandInfo {
            cmd: command,
            exec_name,
            args,
        })
    } else {
        eprintln!("{}", command.get_usage());
        None
    }
}