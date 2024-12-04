use serialport;
use std::time::Duration;

fn main() {
    // Set up the port information (TODO: Make this auto-detected)
    let port = "COM4";
    let baudrate = 9600;

    // Establish connection to serial port
    let mut serial_port = serialport::new(port, baudrate)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Failed to open serial port");

    // Infinite Loop continuously reading serial_buf
    loop {
        // buffer to hold the information read
        let mut serial_buf: Vec<u8> = vec![0; 32];

        // Read the serial info
        match serial_port.read(serial_buf.as_mut_slice()) {
            Ok(bytes_read) => { // if execution ok
                if bytes_read == 0 {
                    continue;
                }

                // Print to terminal (Don't keep this)
                for byte in &serial_buf[..bytes_read] {
                    print!("{:02X} ", byte);
                }
                println!();  // Newline after each read

                //todo: implementation will go here
            }, 
            Err(e) => {
                // if no information available it will throw an error, ignore it.
                if e.kind() == std::io::ErrorKind::TimedOut {
                    continue;
                }

                // else, print the error
                eprintln!("Error reading from serial port: {}", e);
            }
        }            
    }

}