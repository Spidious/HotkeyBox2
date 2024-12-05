use serde::{Deserialize, Serialize};
use std::fs;

// Create a macro for testing the bit at position n
macro_rules! test_bit {
    ($bits:expr, $n:expr) => {
        ($bits >> $n & 1) == 1
    };
}


// Hold the json button info.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ButtonData {
    id: u8,
    action_type: String,
    action: String,
}

#[derive(Deserialize)]
struct ButtonList {
    buttons: Vec<ButtonData>,
}
    
pub fn message_handler(msg: u32){
    // Grab the message (inner 16 bits)
    let content: u16 = (msg >> 8 & 0xFFFF) as u16;

    // Grab the flags
    let _flags: u8 = (content >> 12 & 0xF) as u8;

    // Grab the buttons
    let buttons: u8 = (content >> 4 & 0xFF) as u8;

    // Grab the extra commands
    let _extra: u8 = (content & 0xF) as u8;

    // todo: Read and handle the flags

    // Loop through each button, print the list of buttons being pressed
    for i in (0..8).rev() {
        // Test the bit at position i
        if test_bit!(buttons, i) {
            // Get button attributes and call output
            match get_button_fn(8-i) {
                Some(button) => button_handler(button),
                None => eprintln!("Button not found"),
            }
        }
    }

    // todo: Read and handle the extra calls
}

/// <h2> Get Button Funciton </h2>
/// get the button functionality from the json
/// Fills and returns a ButtonData struct
fn get_button_fn(button_id: u8) -> Option<ButtonData> {
    let file_path = "resources/btn_info.json";

    // Read the file content
    let data = fs::read_to_string(file_path).expect("Unable to read file");

    // Deserialize the JSON into a ButtonList
    let button_list: ButtonList = serde_json::from_str(&data).unwrap();

    // Find the button by id and return a clone
    for button in button_list.buttons.iter() {
        if button.id == button_id {
            return Some(button.clone()); // Clone the ButtonData
        }
    }

    None
}

/// <h2> Handle the button actions </h2>
/// Run whatever the button is supposed to do
fn button_handler(btn: ButtonData) {
    // todo: Actually handle button actions
    println!("Action: {}", btn.action);
}