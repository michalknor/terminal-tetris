mod ui;
mod engine;
mod utils;

use utils::terminal::colorize::{colorize_text, colorize_background, Colors};


fn main() {
    ui::ui::main();

    // println!("{}", colorize_background(colorize_text("AHOJ".to_string(), Colors::YELLOW), Colors::GRAY));
}