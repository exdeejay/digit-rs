use dserver::conn::Connection;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() {
    let mut connections = Vec::new();
    let ip = "127.0.0.1:1234";
    let listener = TcpListener::bind(ip).await.unwrap();
    println!("running on {}", ip);
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        let conn = Arc::new(Connection::new(socket, addr));
        connections.push(conn.clone());
        tokio::spawn(async move {
            handle_client(conn).await;
        });
    }
}

pub async fn handle_client(conn: Arc<Connection>) {
	 
}
