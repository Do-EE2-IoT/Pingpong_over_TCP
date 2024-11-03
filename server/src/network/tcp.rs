use std::str;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
pub struct ServerTcp {
    pub socket: TcpStream,
}

impl ServerTcp {
    // Hàm tạo server, lắng nghe tại địa chỉ `addr`
    pub async fn bind_and_accept(addr: &str) -> Result<Self, io::Error> {
        let listener = TcpListener::bind(addr).await?;
        let (socket, _) = listener.accept().await?;

        Ok(ServerTcp { socket })
    }

    pub async fn receive_data(&mut self) -> Result<String, io::Error> {
        let mut buffer = vec![0; 1024]; // Tạo một buffer để lưu dữ liệu nhận được

        let n = self.socket.read(&mut buffer).await?;

        match str::from_utf8(&buffer[..n]) {
            Ok(string) => Ok(string.to_string()),
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid UTF-8 sequence",
            )),
        }
    }

    pub async fn respond(&mut self, message: &str) -> Result<(), io::Error> {
        self.socket.write_all(message.as_bytes()).await?;
        Ok(())
    }
}
