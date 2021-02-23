use bytes::Bytes;
use mini_redis::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", Bytes::from("world")).await?;

    let result = client.get("hello").await?;

    println!("get value from server; result = {:?}", result);

    Ok(())
}