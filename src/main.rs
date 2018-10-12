extern crate reqwest;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

mod rofi;
mod todoist;

use std::process::exit;

fn main() {
    let option = match rofi::get_text("Add Task") {
        Some(text) => text,
        None => {
            println!("No text, exiting");
            exit(1);
        }
    };

    let response_message = match todoist::create_task(option) {
        Ok(_) => String::from("Success"),
        Err(e) => format!("Error: {}", e),
    };

    rofi::show_message(response_message);
}
