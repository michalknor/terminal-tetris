use std::io::{self, Write};

pub fn hide_cursor() {
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();
}