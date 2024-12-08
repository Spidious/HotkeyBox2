use handler::message_handler;
use serialport::{self, available_ports, SerialPortType};
use std::time::Duration;
mod handler;

fn main() {   
    loop {
        // List ports for debugging purposes
        find_port();

        // Set up the port information
        let port = "COM4";
        let baudrate = 9600;

        // Establish connection to serial port
        let mut serial_port = serialport::new(port, baudrate)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Failed to open serial port");

        // Define a msg variable that will hold the message variable
        let mut msg: u32 = 0;

        // Pre-allocate a fixed-size buffer for reading
        let mut serial_buf = [0u8; 1];

        // Infinite loop continuously reading serial_buf
        loop {
            // Read the serial info
            match serial_port.read(&mut serial_buf) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        // Slow the loop down
                        std::thread::sleep(Duration::from_millis(10));
                        continue;
                    }

                    // Make room for the new byte in the message
                    msg = (msg << 8) | serial_buf[0] as u32;

                    // Check if msg contains the beginning and ending markers
                    if ((msg >> 24 & 0xFF) == 0xC2) && ((msg & 0xFF) == 0x43) {
                        // Send verified message to handler
                        message_handler(msg);

                        // Debugging print
                        // println!("{:x}", msg);

                        // Continue to process potential additional messages
                    }
                }
                Err(e) => {
                    // Ignore timeout errors
                    if e.kind() == std::io::ErrorKind::TimedOut {
                        continue;
                    }

                    // Print other errors
                    println!("Error reading from serial port: {}", e);
                    break;
                }
            }
        }

        // If the inner loop fails try again in like 1/2 of a second
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn find_port() {
    // Find all available ports
    match available_ports() {
        Ok(ports) => {
            // Loop through each port 
            //todo: Change this to be done in threading
            for port in ports {
                let mut serial_buf = [0u8; 1];

                // Connect to that port
                let mut serial_port = serialport::new(port.port_name, 9600)
                        .timeout(Duration::from_millis(1000))
                        .open()
                        .expect("Failed to open serial port");

                // listen to the comport for 2 seconds
                loop {
                    match serial_port.read(&mut serial_buf) {
                        Ok(bytes_read) =>{}
                        Err(e) => {}
                    }

                    //todo: Make it actually only run for 2 seconds and not just once
                    break;
                }
            }
        }
        Err(e) => eprintln!("Error listing ports: {}", e),
    }
}