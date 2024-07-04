use redis::{Commands, RedisError};

#[tokio::main]
async fn main() -> Result<(), RedisError> {
    let client = redis::Client::open("redis://localhost:6379")?;

    let mut pub_con = client.get_connection()?;
    let mut sub_con = client.get_connection()?;
    let mut pub_sub = sub_con.as_pubsub();

    let channel = String::from("Test1");

    pub_sub.subscribe(&channel)?;

    pub_con.publish(&channel, String::from("Hello, Redis!"))?;

    loop {
        let msg = pub_sub.get_message()?;
        let payload: String = msg.get_payload()?;
        let channel: String = msg.get_channel()?;
        println!("Channel '{}': {}", channel, payload);
    }
}
