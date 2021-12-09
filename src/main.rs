use mini_redis::{client, Result};

mod models;
use crate::models::{index_price_for};

#[tokio::main]
async fn main() -> Result<()> {

    let mut client = client::connect("127.0.0.1:6379").await?;

    let symbols : [&str; 3] = ["SOL", "ETH", "BTC"];

    for symbol in symbols {

        index_price_for(symbol, &mut client).await;
   
    }

    
    Ok(())
}


