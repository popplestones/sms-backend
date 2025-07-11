use redis::AsyncCommands;
use sms_core::models::sms::SmsMessage;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_multiplexed_async_connection().await?;

    println!("Worker listening on sms_queue");

    loop {
        let (_key, payload): (String, String) = con.blpop("sms_queue", 0.0).await?;

        match serde_json::from_str::<SmsMessage>(&payload) {
            Ok(msg) => {
                println!("Received message: {msg:?}");
            }
            Err(e) => {
                println!("Error parsing message: {e:?}");
            }
        }
    }
}
