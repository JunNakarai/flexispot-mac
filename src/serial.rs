use std::time::Duration;
use std::io::Write;

pub fn send_command(port_path: &str, command: &[u8]) -> Result<(), String> {
    let mut port = serialport::new(port_path, 9600)
        .timeout(Duration::from_millis(100))
        .open()
        .map_err(|e| format!("Failed to open port: {}", e))?;

    port.write_all(command)
        .map_err(|e| format!("Failed to write to port: {}", e))?;
    port.flush()
        .map_err(|e| format!("Flush failed: {}", e))?;

    std::thread::sleep(Duration::from_millis(100));

    Ok(())
}