// Basic get and post methods

/* 
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// This is the function called on the get method
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// This is the function called on the post method
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
*/

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "User")]
struct User {
    user_name: String,
    user_id: i64,
    user_character: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "Player")]
struct Player {
    player_name: String,
    player_id: i64,
    player_level: i64,
    player_state: PlayerState,
    user_ref: Box<User>,
}

#[derive(Serialize, Deserialize)]
enum PlayerState {
    Live,
    Dead,
}

#[post("/create")]
async fn new(data: String) -> impl Responder {
    let player = serde_json::to_string(&new_player(data)).unwrap();
    HttpResponse::Ok().body(player)
}

fn new_player(json: String) -> Player {
    println!("{}", json);
    let test: Player = serde_json::from_str(&json.as_str()).unwrap();

    return test;
}
/**
impl Player {
    fn kill(mut self: Player) -> Player {
        match self.player_state {
            PlayerState::Live => {
                self.player_state = PlayerState::Dead;
                return self;
            },
            PlayerState::Dead => {return self},
        }
    }
}*/

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard
    dbg!("{counter}");
    
    format!("Request number: {counter}") // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .service(new)
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
