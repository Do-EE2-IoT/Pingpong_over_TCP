use crossterm::event::{self, Event, KeyCode};
use serde::{Serialize, Deserialize};
use std::{io, time::Duration};
use tokio::time::{self};

#[derive(Serialize, Deserialize)]
pub enum UserCommand {
    Up,
    Down,
    None,
}

pub async fn get_input_command() -> Result<UserCommand, io::Error> {
    time::sleep(Duration::from_millis(10)).await;

    if event::poll(Duration::from_millis(0))? {
        if let Event::Key(key_event) = event::read()? {
            return match key_event.code {
                KeyCode::Up => Ok(UserCommand::Up),
                KeyCode::Down => Ok(UserCommand::Down),
                _ => Ok(UserCommand::None),
            };
        }
    }

    Ok(UserCommand::None)
}
