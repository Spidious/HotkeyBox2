use handler::message_handler;
use serialport;
use std::time::Duration;
mod handler;

/// <h2> Vector Pop </h2>
/// Pop the first element off of the vector.<br>
/// <b>vector</b> <i>Vec&ltu8&gt</i> : 8 bit integer vector<br>
/// <b>return:</b> <i>u8</i> : 8 bit inter value
fn pop_vec(vector: &mut Vec<u8>) -> u8{
    // Create a first value and set it to zero
    let mut first: u8 = 0;

    // if the vector is not empty, remove the first value and set first to it
    if !vector.is_empty() {
        first = vector.remove(0);
    }

    // return either zero or popped value
    return first;
}

 
fn main() {
    // Set up the port information
    //todo: Make this auto-detected
    let port = "COM4";
    let baudrate = 9600;

    // Establish connection to serial port
    let mut serial_port = serialport::new(port, baudrate)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Failed to open serial port");

    // Define a msg variable that will hold the message variable
    let mut msg: u32 = 0;

    
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
                while !serial_buf.is_empty() {
                    // make room for new byte
                    msg = msg << 8;

                    // add new byte to end of msg
                    msg |= pop_vec(&mut serial_buf) as u32;
                    
                    // Check if msg contains the beginning end ending markers

                    if (msg >> 24 & 0xFF) == 0xC2 && (msg & 0xFF) == 0x43 {
                        // Send verified message to handler
                        message_handler(msg);
                        
                        // Debugging print
                        // println!("{:x}", msg);

                        // From here the code should continue to the next round incase it caught the start or entirety of another message
                    }
                }
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