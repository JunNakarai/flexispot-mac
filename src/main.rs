use std::collections::HashMap;
use std::time::Duration;
use std::io::Write;
use std::thread::sleep;

fn supported_commands() -> HashMap<&'static str, &'static [u8]> {
    HashMap::from([
        ("up",       &b"\x9b\x06\x02\x01\x00\xfc\xa0\x9d"[..]),
        ("down",     &b"\x9b\x06\x02\x02\x00\x0c\xa0\x9d"[..]),
        ("preset_1", &b"\x9b\x06\x02\x04\x00\xac\xa3\x9d"[..]),
        ("preset_2", &b"\x9b\x06\x02\x08\x00\xac\xa6\x9d"[..]),
        ("stand",    &b"\x9b\x06\x02\x10\x00\xac\xac\x9d"[..]),
        ("sit",      &b"\x9b\x06\x02\x00\x01\xac\x60\x9d"[..]),
    ])
}

fn main(){
    let port_path = "/dev/tty.usbserial-FTAOF5B9";
    let command_key = "up";

    let commands = supported_commands();
    let command = match commands.get(command_key){
        Some(cmd) => cmd,
        None => {
            eprintln!("Command not supported: {}", command_key);
            return;
        }
    };

    let mut port = match serialport::new(port_path,9600)
    .timeout(Duration::from_millis(100))
    .open(){
        Ok(port) => port,
        Err(e) => {
            eprintln!("Failed to open port: {}", e);
            return;
        }
    };

    if let Err(e) = port.write_all(command) {
        eprintln!("Failed to write to port: {}", e);
        return;
    }
    if let Err(e) = port.flush() {
        eprintln!("Flush failed: {}", e);
    }
    sleep(Duration::from_millis(100));
}