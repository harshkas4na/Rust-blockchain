mod block;
mod blockchain;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use blockchain::Blockchain;
use std::sync::{Arc, Mutex};
use redis::{Commands, RedisError, Client};
use tokio::task;

struct AppState {
    blockchain: Mutex<Blockchain>,
    redis_client: Client,
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

    let mut redis_conn = data.redis_client.get_connection().unwrap();
    let _: () = redis_conn.publish("blockchain_updates", "A new block has been added").unwrap();

    HttpResponse::Ok().json(&*blockchain.chain)
}

async fn is_valid(data: web::Data<AppState>) -> impl Responder {
    let blockchain = data.blockchain.lock().unwrap();
    let validity = blockchain.is_valid();
    HttpResponse::Ok().json(validity)
}

async fn redis_subscriber(client: Client) -> Result<(), RedisError> {
    let con = Arc::new(Mutex::new(client.get_connection()?));
    let pub_sub = con.clone();

    task::spawn(async move {
        let mut pub_sub = pub_sub.lock().unwrap();
        let mut pub_sub = pub_sub.as_pubsub();
        pub_sub.subscribe("blockchain_updates").unwrap();

        loop {
            let msg = pub_sub.get_message().unwrap();
            let payload: String = msg.get_payload().unwrap();
            println!("Received notification: {}", payload);
        }
    });

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_client = Client::open("redis://localhost:6379").unwrap();
    redis_subscriber(redis_client.clone()).await.unwrap();

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

    let app_state = web::Data::new(AppState {
        blockchain: Mutex::new(my_blockchain),
        redis_client,
    });

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
