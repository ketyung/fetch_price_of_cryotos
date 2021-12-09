use mini_redis::{client, Result};

mod models;
use crate::models::{index_price_for};

#[tokio::main]
async fn main() -> Result<()> {

    let mut client = client::connect("127.0.0.1:6379").await?;

    index_price_for("SOL", &mut client).await;
    
    Ok(())
}


