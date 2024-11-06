mod cmd_in;
mod game;
use async_std::task::spawn;
use cmd_in::{get_input_command, UserCommand};
use game::pingpong::{game_pingpong_run, pingpong_update, GameData};
use library::network::tcp::client_tcp::ClientTcp;
use std::io;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let mut client_tcp = ClientTcp::connect("127.0.0.1:8080").await.unwrap();

    let (tx, rx): (Sender<GameData>, Receiver<GameData>) = tokio::sync::mpsc::channel(100);
    spawn(async move {
        game_pingpong_run(rx);
    });
    sleep(Duration::from_secs(2)).await;
    loop {
        tokio::select! {
            get_cmd = get_input_command() => {
                match get_cmd {
                    Ok(command) => match command {
                        UserCommand::Up => {
                            // Tạo JSON cho lệnh "up"
                            let data = serde_json::to_vec(&UserCommand::Up)?;
                            client_tcp.send_data(data).await?;
                          //  println!("UP");
                        },
                        UserCommand::Down => {
                            let data = serde_json::to_vec(&UserCommand::Down)?;
                            client_tcp.send_data(data).await?;
                        },
                        UserCommand::Get => {
                            let data = serde_json::to_vec(&UserCommand::Get)?;
                            client_tcp.send_data(data).await?;
                        },
                    },
                    Err(e) => {
                        eprintln!("Failed to get command: {:?}", e);
                    }

                }
            },
             // Nhận dữ liệu từ server và xử lý
             recv_result = client_tcp.receive_data() => {
                match recv_result {
                    Ok(data) => {
                            pingpong_update(tx.clone(),data).await?;
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
