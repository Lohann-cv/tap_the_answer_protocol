use crate::server_struct::{ServerStatus, User};
use crate::tcp_command;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::OwnedSemaphorePermit;
use tokio::sync::{RwLock, mpsc};
use tokio::time::sleep;

pub async fn init_user(
    server_status: &mut ServerStatus,
    mut socket: TcpStream,
    permit: OwnedSemaphorePermit,
) -> Result<User, Box<dyn Error + Send>> {
    let mut connexion_count = server_status.connected_users.write().await;
    *connexion_count += 1;
    let message = format!("OK hello proto={}\n", *connexion_count);
    handle_tcp_message(&mut socket, Message::Write(message)).await?;
    Ok(server_status.init_user(socket, permit, server_status.mpsc_tx.clone()))
}

#[derive(Debug)]
enum Message {
    Write(String),
    Read,
}

pub async fn client_handler(mut user: User) -> Result<(User, String), Box<dyn Error + Send>> {
    let name = authentication(&mut user.socket, user.mpsc_tx.clone()).await?;
    println!("END AUTH");
    let (tx_mpsc, mut rx_mpsc) = mpsc::channel(100); // TEMP MAGIC NUMBER
    {
        let mut writer = user.mpsc_tx.write().await;
        writer.insert(name.clone(), tx_mpsc.clone());
    }
    loop {
        tokio::select! {
            message = handle_tcp_message(&mut user.socket, Message::Read) => {
                println!("{:?}", message);
                println!("OK CLI");
                let res = handle_tcp_action(message.unwrap()).await?;
                if res.contains("QUIT") {
                    break;
                }
                println!("AFTER HANDLE");
                /*In case of QUIT command shutdown gracefully*/
            }

            Some(message) = rx_mpsc.recv() => {
                println!("OK PRIVATE");
                let _ = handle_tcp_action(message).await;
            }

            Ok(message) = user.broadcast_rx.recv() => {
                println!("OK PUB");
                let _ = handle_tcp_action(message).await;
            }
        }
    }
    Ok((user, name))
}

async fn authentication(
    socket: &mut TcpStream,
    rx_mpsc: Arc<RwLock<HashMap<String, mpsc::Sender<String>>>>,
) -> Result<String, Box<dyn Error + Send>> {
    println!("IN AUTH");
    loop {
        let name = handle_tcp_message(socket, Message::Read).await?;
        if !name.contains("CONNECT") {
            handle_tcp_message(
                socket,
                Message::Write(String::from("ERR 400 INVALID_COMMAND\n")),
            )
            .await?;
            continue;
        }
        let reader = rx_mpsc.read().await;
        if reader.contains_key(&name) {
            println!("NOOOOOOOn");
            handle_tcp_message(
                socket,
                Message::Write(String::from("ERR 201 NAME_IN_USE\n")),
            )
            .await?;
        } else {
            println!("OUIIIIIIIII");
            handle_tcp_message(socket, Message::Write(String::from("OK connected\n"))).await?;
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
            println!("WROTE : {}", message);
            Ok(String::new())
        }
        Message::Read => {
            let mut reader = BufReader::new(socket);
            let mut line = String::new();
            let n = reader
                .read_line(&mut line)
                .await
                .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
            if n == 0 {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::ConnectionReset,
                    "Client disconnected",
                )));
            }
            println!("READED : {}", line);
            Ok(line)
        }
    }
}

pub async fn shutdown(mut user: User, name: String) {
    let mut connexion_count = user.connected_users.write().await;
    *connexion_count -= 1;
    user.socket.shutdown().await.unwrap(); // TODO => Handle error
    let mut writer = user.mpsc_tx.write().await;
    writer.remove(&name);
}

async fn handle_tcp_action(message: String) -> Result<String, Box<dyn Error + Send>> {
    println!("Reached handler");
    if message.contains("QUESTS") {
        tcp_command::quests(message).await
    } else if message.contains("QUEST") {
        tcp_command::quest(message).await
    } else if message.contains("GROUPE CREATE") {
        tcp_command::group_create(message).await
    } else if message.contains("GROUPE INVITE") {
        tcp_command::group_invite(message).await
    } else if message.contains("GROUPE JOIN") {
        tcp_command::group_join(message).await
    } else if message.contains("GROUPE LEAVE") {
        tcp_command::group_leave(message).await
    } else if message.contains("LOOK") {
        tcp_command::look(message).await
    } else if message.contains("MOVE") {
        tcp_command::move_command(message).await
    } else if message.contains("QUIT") {
        tcp_command::quit(message).await
    } else if message.contains("CHAT") {
        tcp_command::chat(message).await
    } else if message.contains("WHO") {
        tcp_command::who(message).await
    } else if message.contains("TAKE") {
        tcp_command::take(message).await
    } else if message.contains("INVENTORY") {
        tcp_command::inventory(message).await
    } else if message.contains("TALK") {
        tcp_command::talk(message).await
    } else if message.contains("DROP") {
        tcp_command::drop_command(message).await
    } else if message.contains("ATTACK") {
        tcp_command::attack(message).await
    } else if message.contains("STATUS") {
        tcp_command::status(message).await
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "ERR 400 INVALID_COMMAND",
        )))
    }
}
