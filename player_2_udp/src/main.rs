use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
mod game;
use async_std::task::spawn;
use game::pingpong::{game_pingpong_run, pingpong_update, GameData, UserCommand};
use library::network::udp::UDP;
use tokio::select;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Khởi tạo server và chấp nhận kết nối
    let mut socket = UDP::listen("0.0.0.0:8080").await.unwrap();
    let (tx, rx): (Sender<UserCommand>, Receiver<UserCommand>) = tokio::sync::mpsc::channel(100);
    let (tx_game_data, mut rx_game_data): (Sender<GameData>, Receiver<GameData>) =
        tokio::sync::mpsc::channel(100);
    spawn(async move {
        game_pingpong_run(rx, tx_game_data.clone());
    });
    let address = SocketAddrV4::new(Ipv4Addr::new(172, 16, 100, 196), 8080);
    // Lặp để nhận dữ liệu từ client và cập nhật game
    loop {
        select! {
            // Nhận dữ liệu từ client
            result = socket.read() => {
                match result {
                    Ok(data)=> {
                        if let Err(err) =   pingpong_update(tx.clone(),data).await{
                            println!("Can't update data with {:?}", err);
                        }
                    },

                    Err(e) => {
                        panic!("{e}");
                    }
                }
            },

             // Handle incoming game data
            Some(game_data) = rx_game_data.recv() => {
                match serde_json::to_vec(&game_data) {
                    Ok(data) => {
                        if let Err(e) = socket.send(&address, data).await{
                            println!("Fail to send message over udp {e}");
                        }
                    }
                    Err(e) => eprintln!("Failed to serialize GameData to JSON: {:?}", e),
                }
            },



        }
    }

    Ok(())
}
