use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct ClientTcp {
    stream: TcpStream,
}

impl ClientTcp {
    pub async fn connect(addr: &str) -> Result<Self, io::Error> {
        let stream = TcpStream::connect(addr).await.unwrap();
        Ok(ClientTcp { stream })
    }

    pub async fn send_data(&mut self, data: &[u8]) -> Result<(), io::Error> {
        self.stream.write_all(data).await?;
        Ok(())
    }

    pub async fn receive_data(&mut self, buffer: &mut [u8]) -> Result<usize, io::Error> {
        self.stream.read(buffer).await
    }
}
