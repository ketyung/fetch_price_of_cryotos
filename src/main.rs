use mini_redis::{client, Result};

mod models;
use crate::models::{get_price_of, PRICE_PREFIX, CurrencyPrice};
use std::time::SystemTime;


#[tokio::main]
async fn main() -> Result<()> {

    let mut client = client::connect("127.0.0.1:6379").await?;

    let price_id = format! ("{}{}", PRICE_PREFIX , "SOLUSD") ;

    let res = client.get( &price_id ).await?;

    match res {

        Some(s) =>{
   
            let cprice : CurrencyPrice = serde_json::from_slice(s.as_ref()).unwrap();

            if cprice.last_updated.unwrap_or(SystemTime::now()).elapsed().unwrap().as_secs() > 3600 {

                let mut prices = get_price_of!(String::from("SOL")).await.unwrap();

               
                let serialized = serialize_and_set_time(&mut prices[0]);

                client.set (&price_id, bytes::Bytes::from(serialized) ).await?;

            }
            else {

                println!("Stored priced is {}", cprice.price);
            }

        }
        ,
        None =>{

            let mut prices = get_price_of!(String::from("SOL")).await.unwrap();

            let serialized = serialize_and_set_time(&mut prices[0]);

            client.set (&price_id, bytes::Bytes::from(serialized) ).await?;

        }
    }

  
    Ok(())

}


fn serialize_and_set_time ( price : &mut CurrencyPrice) -> String {

    price.last_updated = Some(SystemTime::now());

    println!("fetched remote price is ::{}", price.price);

    return serde_json::to_string(&price).unwrap();

}

