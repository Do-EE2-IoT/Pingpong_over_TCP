pub mod client_tcp {
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
}

pub mod server_tcp {
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

        pub async fn receive_data(&mut self) -> Result<Vec<u8>, io::Error> {
            let mut buffer = vec![0; 1024]; // Tạo một buffer để lưu dữ liệu nhận được

            let n = self.socket.read(&mut buffer).await?;

            buffer.truncate(n);
            Ok(buffer)
        }

        pub async fn respond(&mut self, message: Vec<u8>) -> Result<(), io::Error> {
            self.socket.write_all(&message).await?;
            Ok(())
        }
    }
}
