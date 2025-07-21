use std::{thread, time::Duration};

fn print_delay() {
    thread::sleep(Duration::from_millis(30));
}

fn forward_char() {
    for i in 60..70 {
        // Convert the integer to a character
        if let Some(c) = char::from_u32(i as u32) {
            //print newline and space
            print!("\n\x1B[{i}G");
            //print char colored
            print!("\x1B[{}m {c} \x1B[0m", i + 30);
            print_delay();
        }
    }
}

fn backward_char() {
    for i in (62..70).rev() {
        // Convert the integer to a character
        if let Some(c) = char::from_u32(i as u32) {
            //print newline and space
            print!("\n\x1B[{i}G");
            //print char colored
            print!("\x1B[{}m {c} \x1B[0m", i - 40);
            print_delay();
        }
    }
}

fn main() {
    loop {
        forward_char();
        backward_char();
    }
}
