mod cmd_in;
mod game;
use async_std::task::spawn;
use cmd_in::{get_input_command, UserCommand};
use game::pingpong::{game_pingpong_run, pingpong_update, GameData};
use library::network::udp::UDP;
use std::{io, net::SocketAddrV4};
use tokio::sync::mpsc::{Receiver, Sender};

const BROADCAST_ADDRESS: &str = "255.255.:7879";
#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut socket = UDP::listen("0.0.0.0:7878").await.unwrap();
    let (tx, rx): (Sender<GameData>, Receiver<GameData>) = tokio::sync::mpsc::channel(100);
    spawn(async move {
        game_pingpong_run(rx);
    });
    let address :SocketAddrV4 = BROADCAST_ADDRESS.parse().unwrap();
    loop {
        tokio::select! {
                get_cmd = get_input_command() => {
                    match get_cmd{
                        Ok(command) => {
                            match command{
                                UserCommand::Up => {
                                    let data = serde_json::to_vec(&UserCommand::Up)?;
                                    socket.send(&address, data).await.unwrap();
                                },
                                UserCommand::Down => {
                                    let data = serde_json::to_vec(&UserCommand::Down)?;
                                    socket.send(&address, data).await.unwrap();
                                },
                                UserCommand::Get => {
                                    let data = serde_json::to_vec(&UserCommand::Get)?;
                                    socket.send(&address, data).await.unwrap();
                                },
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

// use tokio::net::UdpSocket;
// use std::io;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     // Tạo một socket UDP và bind tới địa chỉ lắng nghe
//     let socket = UdpSocket::bind("0.0.0.0:8080").await?;
//     println!("Server listening on 0.0.0.0:8080");

//     let mut buf = [0u8; 1024];

//     loop {
//         // Nhận dữ liệu từ socket
//         let (len, addr) = socket.recv_from(&mut buf).await?;
//         println!("Received from {}: {:?}", addr, &buf[..len]);
//     }
// }
