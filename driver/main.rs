// Create a macro for testing the bit at position n
macro_rules! test_bit {
    ($bits:expr, $n:expr) => {
        ($bits >> $n & 1) == 1
    };
}

fn main() {
    // create sample msg

    let mut vector = vec![0x00, 0x00, 0xC2, 0x08, 0x80, 0x43, 0x00];

    // Define the message and initialize to zero
    let mut msg: u32 = 0;

    for n in vector.iter_mut() {
        /*
        if vector contains [1, 2, 3, 4, 5]
        by the end of the loop the msg will have done this
        0000
        0001
        0012
        0123
        1234
        2345 <- last iteration
         */

        // Shift current msg to the left by one byte
        msg = msg << 8;

        // add the current element to end of element. 
        msg |= *n;

        if (msg >> 24 & 0xFF) == 0xC2 && (msg & 0xFF) == 0x43 {
            break;
        }
    }

    if (msg >> 24 & 0xFF) != 0xC2 || (msg & 0xFF) != 0x43 {
        return;
    }

    // Grab the message (inner 16 bits)
    let message: u16 = (msg >> 8 & 0xFFFF) as u16;

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
        // Test the bit at position i
        if test_bit!(buttons, i) {
            // todo: Call a function that will handle the functionality for the button being pressed
            print!("{} ", 8-i);
        }
    }
    println!();

    println!("Extra function: {}", extra);
}
