use tokio::net::{TcpListener, TcpStream};
use core::net::SocketAddr;
use tokio::sync::Semaphore;
use std::error::Error;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
mod server_struct;
use server_struct::{User, Env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut connexion_count: u8 = 0;
    dotenvy::dotenv()?;
    let socket = dotenvy::var("SOCKET")?;
    let max_user = dotenvy::var("MAX_USER")?;
    let envlock = Arc::new(RwLock::new(
        server_struct::Env::new()
    ));
    let server = server_struct::User::new(max_user.parse().unwrap_or(1));

    let semaphore = Arc::new(Semaphore::new(max_user.parse().unwrap_or(0)));
    let listener = TcpListener::bind(socket).await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;

        match semaphore.clone().try_acquire_owned() {
            Ok(_permit) => {
                println!("A new client is conected !");
                connexion_count += 1;
                let message = format!("Ok hello proto={}\n", connexion_count);
                let _ = socket.write_all(message.as_bytes()).await?;
                let env = envlock.clone();
                let user = server.subscribe();
                println!("AFTER SUBSCRIBE");
                let _handle = tokio::spawn(async move {
                    println!("IN TASK");
                    let _ = client_handler(socket, addr, env, user).await;
                });
            }
            Err(_) => {
                let _ = socket.write_all(b"ERR Server full\n").await;
                println!("Client {:?} tried to connect but the server is full", addr);
                let _ = socket.shutdown().await;
            }
        }
    };
}

#[derive(Debug)]
enum Message {
    Write(String),
    Read,
}

async fn client_handler(mut socket: TcpStream, _addr: SocketAddr, env: Arc<RwLock<Env>>, mut user_broad: User) -> Result<(), Box<dyn Error>> {
    let name = authentication(&mut socket, env.clone()).await?;
    let (tx_mpsc, mut rx_mpsc) = mpsc::channel(100); // TEMP MAGIC NUMBER
    {
        let mut writer = env.write().await;
        writer.add_user(name, tx_mpsc.clone());
    }
    loop {
        tokio::select! {
            message = handle_tcp_message(&mut socket, Message::Read) => {
                handle_tcp_action(message.unwrap());
                /*In case of QUIT command shutdown gracefully*/
            }

            Some(message) = rx_mpsc.recv() => {
                handle_tcp_action(message);
            }

            Ok(message) = user_broad.rx.recv() => {
                handle_tcp_action(message);
            }
        }
    }
}

async fn authentication(socket: &mut TcpStream, env: Arc<RwLock<Env>>) -> Result<String, Box<dyn Error>> {
    println!("IN AUTH");
    loop {
        let name = handle_tcp_message(socket, Message::Read).await?;
        let reader = env.read().await;
        if reader.mpsc_tx.contains_key(&name) {
            println!("NOOOOOOOn");
            handle_tcp_message(socket, Message::Write(String::from("ERR 201 NAME_IN_USE"))).await?;
        } else {
            println!("OUIIIIIIIII");
            handle_tcp_message(socket, Message::Write(String::from("OK connected"))).await?;
            return Ok(name);
        }
    }

}

async fn handle_tcp_message(socket: &mut TcpStream, mode: Message) -> Result<String, Box<dyn Error>> {
    match mode {
        Message::Write(message) => {
            let _ = socket.write_all(message.as_bytes()).await;
            Ok(String::new())
        },
        Message::Read => {
            let mut buffer = vec![0; 8192]; // PUT OUT MAGIC NUMBER
            let n = socket.read(&mut buffer).await?;
            let res = String::from_utf8_lossy(&buffer[..n]).to_string();
            println!("READED : {}", res);
            Ok(res)
        }
    }
}

async fn handle_tcp_action(_message: String) {
    todo!("Parse TCP action and act accordingly");
}
