use std::env;
use flexispot_mac::run_flexispot_command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        return;
    }
    let command_key = &args[1].as_str();
    let default_port = "/dev/tty.usbserial-FTAOF5B9";
    //let port_path = &args[2].as_str();
    let port_path = if args.len() > 2 {
        &args[2].as_str()
    } else {
        default_port
    };

    if let Err(e) = run_flexispot_command(command_key, port_path) {
        eprintln!("Error: {}", e);
    } else {
        println!("Command {} executed successfully", command_key);
    }
}