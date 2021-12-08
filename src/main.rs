use mini_redis::{client, Result};

mod models;
use crate::models::{get_price_of};

 
#[tokio::main]
async fn main() -> Result<()> {

    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("text1", "This is a test of text1".into()).await?;

    let res = client.get("text1").await.expect("error");

    println!("The text1 value if {:?}", res);


    let prices = get_price_of(String::from("SOL"), String::from("USD")).await?;

    for p in prices {

       
        println!("Current solana price in USD is ::{}",p.price);
    }    

    Ok(())

}


