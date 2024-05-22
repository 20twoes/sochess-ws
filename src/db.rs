use mongodb::{
    Database,
    bson::doc,
};

use crate::game::Game;
use crate::user::User;

pub async fn get_game(db: &Database, game_id: &str) -> Option<Game> {
    let games_coll = db.collection::<Game>("games");
    let filter = doc! { "pid": game_id };
    let result = games_coll.find_one(filter, None).await;
    let game_option: Option<Game> = match result {
        Ok(option) => option,
        Err(err) => {
            tracing::error!("{:?}", err);
            None
        },
    };
    return game_option;
}

pub async fn save_game_move(db: &Database, game: &Game) {
    let games_coll = db.collection::<Game>("games");
    let filter = doc! { "pid": game.pid.clone() };

    let latest_move = game.moves.last().unwrap();
    let update = doc! {
        "$push": {
            "moves": bson::to_bson(latest_move).unwrap(),
        },
    };
    let _ = games_coll.update_one(
        filter,
        update,
        None
    ).await;
}

pub async fn get_user(db: &Database, username: &str) -> Option<User> {
    let user_coll = db.collection::<User>("users");
    let filter = doc! { "name": username };
    let result = user_coll.find_one(filter, None).await;
    return match result {
        Ok(option) => {
            println!("query result: {:?}", option);
            option
        },
        Err(err) => {
            tracing::error!("{:?}", err);
            None
        },
    };
}

pub async fn update_player(db: &Database, game: &Game, user_id: &String) {
    // Validate user
    // NOTE: We may need to make this an atomic transaction
    if let Some(_) = get_user(&db, user_id.as_str()).await {
        // Update game
        let games_coll = db.collection::<Game>("games");
        let filter = doc! { "pid": game.pid.clone() };

        let update = doc! {
            "$set": {
                "player2": user_id,
                "state": bson::to_bson(&game.state).unwrap(),
            },
        };
        let _ = games_coll.update_one(
            filter,
            update,
            None
        ).await;
    } else {
        tracing::error!("User does not exist: {}", user_id);
    }
}
