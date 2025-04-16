pub mod command;
pub mod serial;

pub fn run_flexispot_command(command_key: &str, port_path: &str)-> Result<(), String> {
    let commands = command::supported_commands();
    let command = match commands.get(command_key) {
        Some(cmd) => cmd,
        None => return Err(format!("Command {} not found", command_key)),
    };

    serial::send_command(port_path, command)
        .map_err(|e| format!("Failed to send command {}: {}", command_key, e))?;
    Ok(())
}
