use handler::message_handler;
use serialport::{self, available_ports, SerialPortInfo};
use std::time::Duration;
use std::time::Instant;
use std::io::{Error, ErrorKind};
mod handler;

fn main() {   
    loop {
        let port: String;

        // List ports for debugging purposes
        match find_port() {
            Ok(found_port) => {
                port = found_port.port_name;
            },
            Err(e) => {
                eprintln!("Error: {}", e);
                eprintln!("Trying again in 2.5 seconds");
                
                // Nothing was found, wait 3 seconds and then try again.
                std::thread::sleep(Duration::from_millis(2500));
                continue;
            }
        }

        // Set up the port information
        let baudrate = 9600;

        // Establish connection to serial port
        let mut serial_port = serialport::new(&port, 9600)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Failed to open serial port");
        println!("Connected to {} | {} ....", port, baudrate);

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
                        let handler: u8 = message_handler(msg);

                        if handler == 1 {
                            let broadcast_reply: u32 = 0x3480002C;
                            match serial_port.write_all(&broadcast_reply.to_be_bytes()) {
                                Ok(_) => {continue;}
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                }
                            }
                        }
                    }

                    // // For debugging
                    // match msg {
                    //     0xEE0101EE => println!("Heartbeat reply received."),
                    //     0xEE0202EE => println!("Miss count incremented."),
                    //     0xEE0303EE => println!("Max misses reached, connection assumed lost."),
                    //     0xEE0404EE => println!("Message received but not a heartbeat reply."),
                    //     // 0xEE0505EE => println!("No message available on the serial port."),
                    //     message => {
                    //         if (message >> 24 == 0xEE) && (message & 0xFF == 0xEE) {
                    //             println!("count: {}", message >> 8 & 0xFFFF);
                    //         }
                    //     },
                    // }
                }
                Err(e) => {
                    // Ignore timeout errors
                    if e.kind() == ErrorKind::TimedOut {
                        continue;
                    }

                    // Print other errors
                    eprintln!("Connection to {} lost: {}", port, e);
                    break;
                }
            }
        }

        // If the inner loop fails try again in like 1/2 of a second
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn find_port() -> Result<SerialPortInfo, Error> {
    // Find all available ports
    match available_ports() {
        Ok(ports) => {
            // Loop through each port
            for port in ports {
                println!("Attempting to check port: {}", port.port_name);

                let mut serial_buf = [0u8; 1];

                // Try to connect to the port
                let mut serial_port = serialport::new(&port.port_name, 9600)
                    .timeout(Duration::from_millis(1000))
                    .open()
                    .map_err(|_| Error::new(ErrorKind::Other, "Failed to open serial port"))?;

                let mut msg: u32 = 0; // Initialize the message variable
                let start_time = Instant::now();

                // Listen to the port for 2.5 seconds
                loop {
                    // Read from the serial port
                    match serial_port.read(&mut serial_buf) {
                        Ok(bytes_read) => {
                            if bytes_read == 0 {
                                // Slow down the loop
                                std::thread::sleep(Duration::from_millis(10));
                                continue;
                            }

                            // Shift and append the byte to the message
                            msg = (msg << 8) | serial_buf[0] as u32;

                            // Check if the message matches the expected broadcast message
                            if msg == 0xC28FFF43 {
                                println!("Port {} accepted", port.port_name);
                                return Ok(port); // Found the port
                            }
                        }
                        Err(e) => {
                            // Handle timeout errors gracefully
                            if e.kind() == std::io::ErrorKind::TimedOut {break;}

                            // Return other errors
                            return Err(Error::new(
                                ErrorKind::Other,
                                format!("Error reading from port {}: {}", port.port_name, e),
                            ));
                        }
                    }

                    // Check if the timeout has been reached
                    if start_time.elapsed() > Duration::from_millis(2500) {
                        break; // Exit the loop if no response within 2.5 seconds
                    }
                }
            }

            // If no port responded with the broadcast message
            Err(Error::new(ErrorKind::NotFound, "No port responded with the broadcast message"))
        }
        Err(e) => Err(Error::new(ErrorKind::NotFound, format!("Failed to list ports: {}", e))),
    }
}
