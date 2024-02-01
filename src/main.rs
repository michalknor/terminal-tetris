mod ui;
mod engine;
mod utils;

use utils::terminal::colorize::{colorize, TextColors};


fn main() {
    ui::ui::main();

    // println!("{}", colorize("AHOJ", TextColors::RED));
}
