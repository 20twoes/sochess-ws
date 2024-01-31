use std::{
    sync::{Arc, Mutex},
};
use mongodb::Database;
use tokio::sync::broadcast;

pub type SharedState = Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub fen: Mutex<String>,
    pub tx: broadcast::Sender<String>,
    pub db: Database,
}

