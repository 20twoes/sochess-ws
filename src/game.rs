use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub pid: String,
    //pub moves: String,
}

//pub async fn get_games() -> Vec<Game> {
//
//}

//impl Game {
//    pub async fn create() -> Self {
//        Self {
//
//        }
//    }
//}
