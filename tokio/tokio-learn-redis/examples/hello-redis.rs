use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // mini-redis-server --port 6380
    let mut client = client::connect("127.0.0.1:6380").await?;
    client.set("Hello", "World".into()).await?;
    let result = client.get("Hello").await?;

    println!("Got value from the server; result={:?}", result);

    Ok(())
}
