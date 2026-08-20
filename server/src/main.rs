use server_struct::ServerStatus;
use std::error::Error;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;
mod server;
mod server_struct;
mod tcp_command;
use server::{client_handler, init_user, shutdown};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut server_status = ServerStatus::setup().await.unwrap(); // TODO => Handle error
    let semaphore = Arc::new(Semaphore::new(server_status.max_users.into()));

    loop {
        let (mut socket, addr) = server_status.socket.accept().await?;
        match semaphore.clone().try_acquire_owned() {
            Ok(permit) => {
                let user = init_user(&mut server_status, socket, permit)
                    .await
                    .map_err(|e| e as Box<dyn Error>)?;
                let _handle = tokio::spawn(async move {
                    let (user, name) = client_handler(user).await.unwrap(); // TODO => Handle error
                    shutdown(user, name).await;
                });
            }
            Err(_) => {
                let _ = socket.write_all(b"ERR 800 SERVER_FULL\n").await;
                println!("Client {:?} tried to connect but the server is full", addr);
                let _ = socket.shutdown().await;
            }
        }
    }
}