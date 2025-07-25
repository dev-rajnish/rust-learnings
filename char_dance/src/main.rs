use std::{thread, time::Duration};

fn print_delay() {
    thread::sleep(Duration::from_millis(40));
}

fn print_border(distance_at: u8) {
    print!("\x1B[{}G", distance_at);
    print!("||");
}

fn forward_char() {
    for i in 60..70 {
        // Convert the integer to a character
        if let Some(c) = char::from_u32(i as u32 + '.' as u32) {
            print_border(40);
            //print space
            print!("\x1B[{}G", i);

            //print char colored
            print!("\x1B[{}m {c} \x1B[0m", i + 30);
            // print!("\x1B[{}m {c} \x1B[0m", i + 30);

            print_delay();

            //printing in rev
            print!("\x1B[{}G", 130 - i);
            print!("\x1B[{}m {c} \x1B[0m", i + 30);
            // print!("\x1B[{}m {c} \x1B[0m", i + 30);

            print_delay();
            print_border(40);
            print_border(90);
            println!();

            print_border(90);
            print_border(40);
            println!();

            print_border(90);
        }
    }
}

fn backward_char() {
    for i in (62..70).rev() {
        if let Some(c) = char::from_u32(i as u32) {
            print_border(40);

            //print space
            print!("\x1B[{}G", i);

            //print char colored
            print!("\x1B[{}m {c} \x1B[0m", i - 40);
            // print!("\x1B[{}m {c} \x1B[0m", i - 40);

            print_delay();
            //printing in rev
            print!("\x1B[{}G", 130 - i + 1);
            print!("\x1B[{}m {c} \x1B[0m", i - 40);
            //print!("\x1B[{}m {c} \x1B[0m", i - 40);

            print_delay();
            print_border(40);
            print_border(90);
            println!();
            print_border(40);
            print_border(90);
            println!();

            print_border(90);
        }
    }
}

fn main() {
    loop {
        forward_char();
        backward_char();
    }
}
