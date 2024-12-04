fn main() {
    // create sample input
    let input: u32 = 0xC25d8a43;

    // Check start and end message
    if (((input >> 24) & 0xFF) == 0xC2) && ((input & 0xFF) == 0x43) {
        // Grab the message (inner 16 bits)
        let message: u16 = (input >> 8 & 0xFFFF) as u16;

        // Grab the flags
        let flags: u8 = (message >> 12 & 0xF) as u8;

        // Grab the buttons
        let buttons: u8 = (message >> 4 & 0xFF) as u8;

        // Grab the extra commands
        let extra: u8 = (message & 0xF) as u8;

        // Print the flags
        println!("Flags: {} {} {} {}", flags >> 3 & 1, flags >> 2 & 1, flags >> 1 & 1, flags & 1);

        // Print buttons list
        if buttons != 0 {
            print!("Buttons Pressed: ");
        }

        // Loop through each button, print the list of buttons being pressed
        for i in (0..8).rev() {
            let bit: u8 = (buttons >> i) & 1; // grab the bit for this iteration

            if bit == 1 {
                // todo: Call a function that will handle the functionality for the button being pressed
                print!("{} ", 8-i);
            }
        }
        println!();

        println!("Extra function: {}", extra);
    }
}