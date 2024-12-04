    // Create a macro for testing the bit at position n
macro_rules! test_bit {
    ($bits:expr, $n:expr) => {
        ($bits >> $n & 1) == 1
    };
}
    
pub fn message_handler(msg: u32){
    // Grab the message (inner 16 bits)
    let content: u16 = (msg >> 8 & 0xFFFF) as u16;

    // Grab the flags
    let flags: u8 = (content >> 12 & 0xF) as u8;

    // Grab the buttons
    let buttons: u8 = (content >> 4 & 0xFF) as u8;

    // Grab the extra commands
    let extra: u8 = (content & 0xF) as u8;

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