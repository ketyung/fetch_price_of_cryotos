use mini_redis::{client, Result};

mod models;
use crate::models::{index_price_for};
use std::env;


#[tokio::main]
async fn main() -> Result<()> {

    let args: Vec<String> = env::args().collect();

    let (api_key, currencies, force_refresh) = parse_args(&args);


    let mut client = client::connect("127.0.0.1:6379").await?;

    for curr in currencies {

        index_price_for(api_key, curr, &mut client, force_refresh).await;
   
    }

    
    Ok(())
}



fn parse_args(args: &[String]) -> (&str, Vec<&str>, bool) {
    let api_key = &args[1];
    let symbols = &args[2];

    let mut force_refresh = false ;

    if  args.len() > 3 && args[3] == "force-refresh" {

        force_refresh = true ;
    }

    let splits = symbols.split(",");

    (api_key, splits.collect(), force_refresh)

}