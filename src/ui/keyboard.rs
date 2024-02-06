// Standard libraries
use std::time::Duration;

// 3rd party crates
use crossterm::event::{poll, Event, KeyEventKind};
use crossterm::event::{read, KeyCode};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use super::ui::SnakeGameState;

pub struct KeyboardListener {
    tx_key_event: UnboundedSender<KeyCode>,
    rx_game_state: UnboundedReceiver<SnakeGameState>,
}

impl KeyboardListener {
    pub fn new(
        tx_key_event: UnboundedSender<KeyCode>,
        rx_game_state: UnboundedReceiver<SnakeGameState>,
    ) -> Self {
        Self {
            tx_key_event,
            rx_game_state,
        }
    }
    pub async fn run(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                match self.rx_game_state.try_recv() {
                    Ok(state) => match state {
                        SnakeGameState::SnakeDied | SnakeGameState::Quit => break,
                    },
                    Err(_err) => {
                        if poll(Duration::from_millis(1000)).unwrap() {
                            if let Event::Key(key_event) = read().unwrap() {
                                // println!("{:?}", key_event);
                                if key_event.kind != KeyEventKind::Press {
                                    continue;
                                }
                                if let Err(e) = self.tx_key_event.send(key_event.code) {
                                    eprintln!("Error: {:?}", e);
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}
