use std::collections::HashMap;

use crate::{rocket, types::PlayerData, Answer, Game, };
use rocket::{http::Status, local::Client};

#[test]
fn not_found() {
    let client = Client::new(rocket(None)).unwrap();
    let res = client.get("/this_is_a_bad_request").dispatch();
    assert_eq!(res.status(), Status::NotFound);
}

#[test]
fn heartbeat() {
    let client = Client::new(rocket(None)).unwrap();
    let res = client.get("/api/v1/heartbeat").dispatch();
    assert_eq!(res.status(), Status::Ok);
}

#[test]
fn simple_game() {
    let client = Client::new(rocket(None)).unwrap();
    // Create game as p1
    let p = PlayerData::new("p1");
    let res = client
        .put("/api/v1/game/my_game")
        .body(serde_json::to_string(&p).unwrap())
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    // Join
    let p = PlayerData::new("p2");
    let res = client
        .post("/api/v1/game/my_game")
        .body(serde_json::to_string(&p).unwrap())
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    // P2 answers
    let a = Answer::new("p2", "test");
    let res = client
        .post("/api/v1/game/my_game/answer")
        .body(serde_json::to_string(&a).unwrap())
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    // P1 answers
    let a = Answer::new("p1", "sssss");
    let res = client
        .post("/api/v1/game/my_game/answer")
        .body(serde_json::to_string(&a).unwrap())
        .dispatch();
    assert_eq!(res.status(), Status::Ok);
    // Get the state of the game
    let mut res = client.get("/api/v1/game/my_game").dispatch();
    let game = serde_json::from_str::<Game>(&res.body_string().unwrap()).unwrap();
    assert_eq!(game.rounds.len(), 2);
    assert_eq!(game.previous_round().unwrap().player_one_answer, Some("sssss".to_string()));
    assert_eq!(game.previous_round().unwrap().player_two_answer, Some("test".to_string()));
    insta::assert_debug_snapshot!(game);

}