mod engine;
mod ui;
mod utils;

use tokio::sync::mpsc::unbounded_channel;

use crate::engine::engine::Game;

#[tokio::main(worker_threads = 2)]
async fn main() -> Result<(), std::io::Error> {
    // let (tx_key_event, rx_key_event) = unbounded_channel();

    // let mut main_game = Game::new(rx_key_event);
    // // Run the game
    // main_game.run().await?;

    let mut ui = ui::ui::UI::new()?;
    // println!("\x1b[38;5;209mAAAA\x1b[0m");
    // println!("\x1b[48;5;209mAAAA\x1b[0m");

    ui.run().await?;

    Ok(())
}