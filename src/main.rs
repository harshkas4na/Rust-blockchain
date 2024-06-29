mod block;
mod blockchain;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use blockchain::Blockchain;
use std::sync::Mutex;

struct AppState {
    blockchain: Mutex<Blockchain>,
}

#[derive(Serialize, Deserialize)]
struct AddBlockData {
    data: String,
}

async fn get_chain(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    HttpResponse::Ok().json(&*blockchain.chain)
}

async fn add_block(data: web::Data<AppState>, block_data: web::Json<AddBlockData>) -> impl Responder {
    let mut blockchain = data.blockchain.lock().unwrap();
    blockchain.add_block(block_data.data.clone());
    HttpResponse::Ok().json(&*blockchain.chain)
}

async fn is_valid(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    let validity = blockchain.is_valid();
    HttpResponse::Ok().json(validity)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the blockchain with a few blocks
    let mut my_blockchain = Blockchain::new();
    let mut count = 0;
    loop {
        my_blockchain.add_block("First block after Genesis".to_string());
        my_blockchain.add_block("Second block after Genesis".to_string());
        my_blockchain.add_block("Third block after Genesis".to_string());

        count += 1;
        if count == 3 {
            break;
        }
    }

    // Wrap the blockchain in a Mutex and shared state
    let app_state = web::Data::new(AppState {
        blockchain: Mutex::new(my_blockchain),
    });

    // Start the Actix-web server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/chain", web::get().to(get_chain))
            .route("/add_block", web::post().to(add_block))
            .route("/is_valid", web::get().to(is_valid))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
