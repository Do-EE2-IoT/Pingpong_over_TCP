use serde_json::json;
use std::io;
use std::str::FromStr;

use async_std::task::spawn;
use game::pingpong::{game_pingpong_run, pingpong_update, GameData};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::cmd_in::{get_input_command, Command};
use crate::network::tcp::ClientTcp;

mod cmd_in;
mod game;
mod network;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut client_tcp = ClientTcp::connect("127.0.0.1:7878").await.unwrap();
    let mut buffer = [0; 1024];
    let (tx, rx): (Sender<GameData>, Receiver<GameData>) = tokio::sync::mpsc::channel(100);
    spawn(async move {
        game_pingpong_run(rx);
    });

    loop {
        tokio::select! {
            // Gọi `get_input_command().await` trực tiếp và xử lý kết quả
            get_cmd = get_input_command() => {
                match get_cmd {
                    Ok(command) => match command {
                        Command::Up => {
                            // Tạo JSON cho lệnh "up"
                            let json_data = json!({ "command": "up" }).to_string();
                            client_tcp.send_data(json_data.as_bytes()).await?;
                            println!("UP");
                        },
                        Command::Down => {
                            // Tạo JSON cho lệnh "down"
                            let json_data = json!({ "command": "down" }).to_string();
                            client_tcp.send_data(json_data.as_bytes()).await?;
                            println!("DOWN");
                        },
                        Command::Get => {
                            // Tạo JSON cho lệnh "get"
                            let json_data = json!({ "command": "get" }).to_string();
                            client_tcp.send_data(json_data.as_bytes()).await?;
                            println!("GET");
                        },
                    },
                    Err(e) => {
                        eprintln!("Failed to get command: {:?}", e);
                    }

                }
            },
             // Nhận dữ liệu từ server và xử lý
             recv_result = client_tcp.receive_data(&mut buffer) => {
                match recv_result {
                    Ok(size) => {
                        if size > 0 {
                            let received_data = String::from_utf8_lossy(&buffer[..size]);
                            pingpong_update(tx.clone(),String::from_str(&received_data).unwrap()).await?;
                        }
                    },
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            }




        }
    }
    Ok(())
}
