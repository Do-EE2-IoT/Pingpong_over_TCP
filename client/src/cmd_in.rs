use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Duration};
use tokio::time::{self};


pub enum Command {
    Up,
    Down,
    Get,
}

pub async fn get_input_command() -> Result<Command, io::Error> {
    time::sleep(Duration::from_millis(30)).await;

    if event::poll(Duration::from_millis(0))? {
        if let Event::Key(key_event) = event::read()? {
            return match key_event.code {
                KeyCode::Up => Ok(Command::Up),
                KeyCode::Down => Ok(Command::Down),
                _ => Ok(Command::Get),
            };
        }
    }

    Ok(Command::Get)
}
