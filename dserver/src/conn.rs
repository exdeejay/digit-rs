use std::net::SocketAddr;

use tokio::net::TcpStream;

pub struct Connection {
    socket: TcpStream,
    addr: SocketAddr,
}

impl Connection {
    pub fn new(socket: TcpStream, addr: SocketAddr) -> Connection {
        println!("got connection from {}", addr);
        Connection { socket, addr }
    }

    pub fn socket(&mut self) -> &mut TcpStream {
        &mut self.socket
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }
}
