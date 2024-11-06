use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::cmd_in::UserCommand;

pub struct ClientTcp {
    stream: TcpStream,
}

impl ClientTcp {
    pub async fn connect(addr: &str) -> Result<Self, io::Error> {
        let stream = TcpStream::connect(addr).await.unwrap();
        Ok(ClientTcp { stream })
    }

    pub async fn send_data(&mut self, data: Vec<u8>) -> Result<(), io::Error> {
        self.stream.write_all(&data).await?;
        Ok(())
    }

    pub async fn receive_data(&mut self) -> Result<Vec<u8>, io::Error> {
        let mut buffer = vec![0; 1024]; // Tạo một buffer để lưu dữ liệu nhận được

        let n = self.stream.read(&mut buffer).await?;

        buffer.truncate(n);
        Ok(buffer)
    }
}
