use mongodb::Database;
use std::sync::Arc;
use tokio::sync::broadcast;

pub type SharedState = Arc<AppState>;

#[derive(Debug)]
pub struct AppState {
    pub tx: broadcast::Sender<String>,
    pub db: Database,
}
