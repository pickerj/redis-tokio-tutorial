use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    {
        // Open first connection to mini-redis address
        println!("Making first connection");
        let mut client = client::connect("127.0.0.1:6379").await?;

        // Set the key "hello" with value "world"
        client.set("hello", "world".into()).await?;

        println!("Key \"hello\" set to value \"world\"");
    }
    {
        println!("Making second connection");
        let mut client = client::connect("127.0.0.1:6379").await?;
        // Get the key "hello"
        let result = client.get("hello").await?;

        println!(
            "Got value of key \"hello\" from server: result={:?}",
            result
        );
    }
    Ok(())
}
