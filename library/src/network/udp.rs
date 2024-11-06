use std::{
    io,
    net::{Ipv4Addr, SocketAddrV4},
};
use tokio::net::UdpSocket;

pub struct UDP {
    socket: UdpSocket,
}

impl UDP {
    pub async fn listen(addr: &str) -> Result<Self, io::Error> {
        let socket = UdpSocket::bind(addr).await.unwrap();
        Ok(Self { socket })
    }

    pub async fn send(&mut self, address: &SocketAddrV4, data: Vec<u8>) -> Result<(), io::Error> {
        self.socket.send_to(&data, &address).await?;
        Ok(())
    }

    pub async fn read(&mut self) -> Result<Vec<u8>, io::Error> {
        let mut buf: Vec<u8> = vec![0; 2024];
        let size = self.socket.recv(&mut buf).await?;
        buf.truncate(size);
        Ok(buf)
    }

    pub async fn broadcast(&mut self, port: u16, data: Vec<u8>) -> Result<(), io::Error> {
        let broadcast_address = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), port);
        self.socket.send_to(&data, broadcast_address);
        Ok(())
    }
}
