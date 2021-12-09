use mini_redis::{client, Result};

mod models;
use crate::models::{index_price_for};
use std::env;


#[tokio::main]
async fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let (api_key, currencies) = parse_args(&args);


    let mut client = client::connect("127.0.0.1:6379").await?;

    for curr in currencies {

        index_price_for(api_key, curr, &mut client).await;
   
    }

    
    Ok(())
}



fn parse_args(args: &[String]) -> (&str, Vec<&str>) {
    let api_key = &args[1];
    let symbols = &args[2];

    let splits = symbols.split(",");

    (api_key, splits.collect())

}