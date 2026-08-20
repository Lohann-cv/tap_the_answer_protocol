use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::sync::{RwLock, mpsc};
mod server_struct;
use server_struct::{ServerStatus, User};
mod tcp_command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut server_status = ServerStatus::setup().await.unwrap(); // TODO => Handle error
    let semaphore = Arc::new(Semaphore::new(server_status.max_users.into()));

    loop {
        if let (mut socket, addr) = server_status.socket.accept().await? {
            match semaphore.clone().try_acquire_owned() {
                Ok(permit) => {
                    server_status.connected_users += 1;
                    let message = format!("Ok hello proto={}\n", &server_status.connected_users);
                    let _ = socket.write_all(message.as_bytes()).await?;
                    let names_list = server_status.mpsc_tx.clone();
                    let user = server_status.init_user(socket, permit, names_list);
                    let _handle = tokio::spawn(async move {
                        let _ = client_handler(user).await;
                    });
                }
                Err(_) => {
                    let _ = socket.write_all(b"ERR Server full\n").await;
                    println!("Client {:?} tried to connect but the server is full", addr);
                    let _ = socket.shutdown().await;
                }
            }
        }
    }
}

#[derive(Debug)]
enum Message {
    Write(String),
    Read,
}

async fn client_handler(mut user: User) -> Result<(), Box<dyn Error + Send>> {
    let name = authentication(&mut user.socket, user.mpsc_tx.clone()).await?;
    println!("END AUTH");
    let (tx_mpsc, mut rx_mpsc) = mpsc::channel(100); // TEMP MAGIC NUMBER
    {
        let mut writer = user.mpsc_tx.write().await;
        writer.insert(name, tx_mpsc.clone());
    }
    loop {
        tokio::select! {
            message = handle_tcp_message(&mut user.socket, Message::Read) => {
                println!("{:?}", &message);
                println!("OK CLI");
                handle_tcp_action(message.unwrap()).await;
                println!("AFTER HANDLE");
                /*In case of QUIT command shutdown gracefully*/
            }

            Some(message) = rx_mpsc.recv() => {
                println!("OK PRIVATE");
                handle_tcp_action(message);
            }

            Ok(message) = user.broadcast_rx.recv() => {
                println!("OK PUB");
                handle_tcp_action(message);
            }
        }
    }
}

async fn authentication(
    socket: &mut TcpStream,
    rx_mpsc: Arc<RwLock<HashMap<String, mpsc::Sender<String>>>>,
) -> Result<String, Box<dyn Error + Send>> {
    println!("IN AUTH");
    loop {
        let name = handle_tcp_message(socket, Message::Read).await?;
        let reader = rx_mpsc.read().await;
        if reader.contains_key(&name) {
            println!("NOOOOOOOn");
            handle_tcp_message(socket, Message::Write(String::from("ERR 201 NAME_IN_USE"))).await?;
        } else {
            println!("OUIIIIIIIII");
            handle_tcp_message(socket, Message::Write(String::from("OK connected"))).await?;
            return Ok(name);
        }
    }
}

async fn handle_tcp_message(
    socket: &mut TcpStream,
    mode: Message,
) -> Result<String, Box<dyn Error + Send>> {
    match mode {
        Message::Write(message) => {
            let _ = socket.write_all(message.as_bytes()).await;
            Ok(String::new())
        }
        Message::Read => {
            let mut buffer = vec![0; 8192]; // PUT OUT MAGIC NUMBER
            let n = socket
                .read(&mut buffer)
                .await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
            let res = String::from_utf8_lossy(&buffer[..n]).to_string();
            println!("READED : {}", res);
            Ok(res)
        }
    }
}

async fn handle_tcp_action(message: String) {
    println!("Reached handler");
    if message.contains("QUESTS") {
        tcp_command::quests(message).await;
    } else if message.contains("QUEST") {
        tcp_command::quest(message).await;
    } else if message.contains("GROUPE CREATE") {
        tcp_command::group_create(message).await;
    } else if message.contains("GROUPE INVITE") {
        tcp_command::group_invite(message).await;
    } else if message.contains("GROUPE JOIN") {
        tcp_command::group_join(message).await;
    } else if message.contains("GROUPE LEAVE") {
        tcp_command::group_leave(message).await;
    } else if message.contains("LOOK") {
        tcp_command::look(message).await;
    } else if message.contains("MOVE") {
        tcp_command::move_command(message).await;
    } else if message.contains("QUIT") {
        tcp_command::quit(message).await;
    } else if message.contains("CHAT") {
        tcp_command::chat(message).await;
    } else if message.contains("WHO") {
        tcp_command::who(message).await;
    } else if message.contains("TAKE") {
        tcp_command::take(message).await;
    } else if message.contains("INVENTORY") {
        tcp_command::inventory(message).await;
    } else if message.contains("TALK") {
        tcp_command::talk(message).await;
    } else if message.contains("DROP") {
        tcp_command::drop_command(message).await;
    } else if message.contains("ATTACK") {
        tcp_command::attack(message).await;
    } else if message.contains("STATUS") {
        tcp_command::status(message).await;
    } else {
        return;
    }
}
