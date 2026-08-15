use tokio::sync::mpsc::{Sender};
use std::collections::HashMap;
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct Env {
    pub mpsc_tx: HashMap<String, Sender<String>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            mpsc_tx: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, name: String, tx: Sender<String>) {
        self.mpsc_tx.insert(name, tx);
    }

    /*
    pub fn get_sender(&self, name: String) -> Result<&Sender<String>, Box<dyn Error>> {
        match self.mpsc_tx.get(&name) {
            Some(tx) => Ok(tx),
            None => Err("Name isn't there".into()),
        }
    }
    */
}

#[derive(Debug)]
pub struct User {
    pub tx: broadcast::Sender<String>,
    pub rx: broadcast::Receiver<String>,
}

impl User {
    pub fn new(size: u8) -> Self {
        let (tx, rx) = broadcast::channel(size.into());
        Self {
            tx,
            rx,
        }
    }

    pub fn subscribe(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            rx: self.tx.subscribe(),
        }
    }
}
