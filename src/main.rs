use std::env;
use flexispot_mac::run_frexispot_command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command>", args[0]);
        return;
    }
    let command_key = &args[1].as_str();
    
    if let Err(e) = run_frexispot_command(command_key) {
        eprintln!("Error: {}", e);
    } else {
        println!("Command {} executed successfully", command_key);
    }
}