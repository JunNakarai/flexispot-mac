mod command;
mod serial;
use std::env;

fn main() {
    let port_path = "/dev/tty.usbserial-FTAOF5B9";
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        return;
    }
    let command_key = &args[1].as_str();
    
    let commands = command::supported_commands();
    let command = match commands.get(command_key){
        Some(cmd) => cmd,
        None => {
            eprintln!("Command not supported: {}", command_key);
            return;
        }
    };

    if let Err(e) = serial::send_command(port_path, command) {
        eprintln!("{}", e);
    }
}