use tokio::net::{TcpListener, TcpStream};
use core::net::SocketAddr;
use tokio::sync::Semaphore;
use std::error::Error;
use tokio::io::AsyncWriteExt;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    let socket = dotenvy::var("SOCKET")?;
    let max_user = dotenvy::var("MAX_USER")?;

    let semaphore = Arc::new(Semaphore::new(max_user.parse().unwrap_or(0)));
    let listener = TcpListener::bind(socket).await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;

        match semaphore.clone().try_acquire_owned() {
            Ok(_permit) => {
                println!("A new client is conected !");
                let _handle = tokio::spawn(async move {
                    client_handler(socket, addr).await;
                });
            }
            Err(_) => {
                let _ = socket.write_all(b"Error: Server full\n").await;
                println!("Client {:?} tried to connect but the server is full", addr);
                let _ = socket.shutdown().await;
            }
        }
    };
}

async fn client_handler(_socket: TcpStream, _addr: SocketAddr) {
    todo!("Creating a reading and writing logic for the client/server comunicaton");
}
