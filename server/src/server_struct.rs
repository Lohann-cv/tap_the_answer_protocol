use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::sync::mpsc;
use tokio::sync::{OwnedSemaphorePermit, RwLock};

#[derive(Debug)]
pub struct ServerStatus {
    pub mpsc_tx: Arc<RwLock<HashMap<String, mpsc::Sender<String>>>>,
    pub broadcast_handle: (Sender<String>, Receiver<String>),
    pub connected_users: u8,
    pub max_users: u8,
    pub socket: TcpListener,
}

impl ServerStatus {
    pub async fn setup() -> Result<Self, Box<dyn Error>> {
        dotenvy::dotenv()?;
        let socket = dotenvy::var("SOCKET")?;
        let max_user: u8 = dotenvy::var("MAX_USER")?.parse()?;
        let listener = TcpListener::bind(socket).await?;
        Ok(Self {
            mpsc_tx: Arc::new(RwLock::new(HashMap::new())),
            broadcast_handle: broadcast::channel(max_user.into()),
            connected_users: 0,
            max_users: max_user,
            socket: listener,
        })
    }

    pub fn init_user(
        &self,
        socket: TcpStream,
        permit: OwnedSemaphorePermit,
        mpsc_tx: Arc<RwLock<HashMap<String, mpsc::Sender<String>>>>,
    ) -> User {
        User {
            socket,
            permit,
            mpsc_tx,
            broadcast_tx: self.broadcast_handle.0.clone(),
            broadcast_rx: self.broadcast_handle.0.subscribe(),
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub socket: TcpStream,
    pub permit: OwnedSemaphorePermit,
    pub mpsc_tx: Arc<RwLock<HashMap<String, mpsc::Sender<String>>>>,
    pub broadcast_tx: Sender<String>,
    pub broadcast_rx: Receiver<String>,
}
