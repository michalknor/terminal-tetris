use std::io::{self, Write};
// use tokio::sync::mpsc::unbounded_channel;

mod engine;
mod ui;
mod utils;

use ui::music::play_theme;


#[tokio::main(worker_threads = 2)]
async fn main() -> Result<(), std::io::Error> {
    hide_cursor();
    // let (tx_key_event, rx_key_event) = unbounded_channel();

    // let mut main_game = Game::new(rx_key_event);
    // // Run the game
    // main_game.run().await?;

    let mut ui = ui::ui::UI::new()?;
    // println!("\x1b[38;5;209mAAAA\x1b[0m");
    // println!("\x1b[48;5;209mAAAA\x1b[0m");
    play_theme().await?;

    ui.run().await?;

    Ok(())
}


fn hide_cursor() {
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();
}