use std::io::{self, Write};
// use tokio::sync::mpsc::unbounded_channel;

mod engine;
mod ui;
mod utils;

use tokio::sync::mpsc::unbounded_channel;
use ui::{keyboard::KeyboardListener, music::play_theme};


#[tokio::main(worker_threads = 3)]
async fn main() -> Result<(), std::io::Error> {
    hide_cursor();
    // let (tx_key_event, rx_key_event) = unbounded_channel();

    // let mut main_game = Game::new(rx_key_event);
    // // Run the game
    // main_game.run().await?;

    let (tx_key_event, rx_key_event) = unbounded_channel();
    let (tx_game_state, rx_game_state) = unbounded_channel();

    // Initialize the keyboard listener
    let keyboard_listener = KeyboardListener::new(tx_key_event, rx_game_state);

    let mut ui = ui::ui::UI::new(rx_key_event, tx_game_state)?;
    // println!("\x1b[38;5;209mAAAA\x1b[0m");
    // println!("\x1b[48;5;209mAAAA\x1b[0m");
    play_theme().await?;
    let keyboard_handler = keyboard_listener.run().await;

    ui.run().await?;

    
    // let _ = tokio::join!(keyboard_handler);

    Ok(())
}


fn hide_cursor() {
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();
}