mod ui;
mod engine;
mod utils;

use utils::terminal::colorize::{colorizeText, colorizeBackground, Colors};


fn main() {
    ui::ui::main();

    println!("{}", colorizeBackground(colorizeText("AHOJ".to_string(), Colors::YELLOW), Colors::GRAY));
}