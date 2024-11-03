use std::io;

use async_std::task::spawn;
use game::pingpong::{game_pingpong_run, pingpong_update, GameData, UserCommand};
use network::tcp::ServerTcp;
use tokio::select;
mod game;
mod network;
use tokio::sync::mpsc::{Receiver, Sender};


#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Khởi tạo server và chấp nhận kết nối
    let mut listener = ServerTcp::bind_and_accept("0.0.0.0:7878").await.unwrap();
    let (tx, rx): (Sender<UserCommand>, Receiver<UserCommand>) = tokio::sync::mpsc::channel(100);
    let (tx_game_data, mut rx_game_data): (Sender<GameData>, Receiver<GameData>) =
        tokio::sync::mpsc::channel(100);
    spawn(async move {
        game_pingpong_run(rx, tx_game_data.clone());
    });

    // Lặp để nhận dữ liệu từ client và cập nhật game
    loop {
        select! {
            // Nhận dữ liệu từ client
            result = listener.receive_data() => {
                match result {
                    Ok(data)=> {
                        pingpong_update(tx.clone(),data).await.unwrap();
                    },

                    Err(e) => {
                        panic!("{e}");
                    }
                }
            },

             // Handle incoming game data
            Some(game_data) = rx_game_data.recv() => {
                match serde_json::to_string(&game_data) {
                    Ok(json) => {
                        if let Err(e) = listener.respond(&json).await {
                            eprintln!("Failed to send response: {:?}", e);
                        }
                    }
                    Err(e) => eprintln!("Failed to serialize GameData to JSON: {:?}", e),
                }
            },

            

        }
    }

    Ok(())
}
