mod cmd_in;
mod game;
use async_std::task::spawn;
use cmd_in::{get_input_command, UserCommand};
use game::pingpong::{game_pingpong_run, pingpong_update, GameData};
use library::network::udp::UDP;
use std::{io, net::SocketAddrV4};
use tokio::sync::mpsc::{Receiver, Sender};


#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut socket = UDP::listen("0.0.0.0:7878").await.unwrap();
    let (tx, rx): (Sender<GameData>, Receiver<GameData>) = tokio::sync::mpsc::channel(100);
    spawn(async move {
        game_pingpong_run(rx);
    });

    loop {
        tokio::select! {
                get_cmd = get_input_command() => {
                    match get_cmd{
                        Ok(command) => {
                            match command{
                                UserCommand::Up => {
                                    let data = serde_json::to_vec(&UserCommand::Up)?;
                                    socket.broadcast(8080, data).await.unwrap();
                                },
                                UserCommand::Down => {
                                    let data = serde_json::to_vec(&UserCommand::Down)?;
                                    socket.broadcast(8080, data).await.unwrap();
                                },
                                UserCommand::None=> {

                                }
                            }
                        },
                        Err(e) => {
                            panic!("Wrong with {e}");
                        }
                    }
                },

                data = socket.read() => {
                    match data {
                        Ok(data) => {
                                pingpong_update(tx.clone(),data).await?;
                        },
                        Err(e) => {
                            panic!("{}", e);
                        }
                    }
            },
        }
    }
    Ok(())
}
