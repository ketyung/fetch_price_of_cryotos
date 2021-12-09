use serde::{Serialize, Deserialize};
use reqwest::Error;
use std::time::SystemTime;
use mini_redis::{client};
use chrono::prelude::DateTime;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyPrice {

    pub currency : String, 

    pub id : String, 

    pub price : String,

    pub name : String, 

    pub last_updated : Option<SystemTime>,
}


pub const PRICE_PREFIX : &str = "nomics_price_";




#[macro_export]
macro_rules! get_price_of {
    
    ($a : expr, $b : expr ) => {
        
        get_price_of($a , $b,  "USD" )
    };
}


pub async fn get_price_of(api_key : &str,  currency : &str, convert_to : &str) -> Result<Vec<CurrencyPrice>, Error>  {

    let request_url = format!(
        "https://api.nomics.com/v1/currencies/ticker?key={key}&ids={ids}&convert={convert}&interval={interval}", 

        key = api_key, 

        ids = currency,

        convert = convert_to,

        interval = "1h",
    );
    
    println!("{}", request_url);


    let response = reqwest::get(&request_url).await?;

    let prices : Vec<CurrencyPrice>  = response.json().await?;

    Ok(prices)
}



pub async fn index_price_for (api_key : &str, currency : &str, 
    client : &mut client::Client, force_refresh : bool) {

    let price_id = format! ("{}{}{}", PRICE_PREFIX , currency, "USD") ;

    let res = client.get( &price_id ).await.unwrap();

    match res {

        Some(s) =>{
   
            let cprice : CurrencyPrice = serde_json::from_slice(s.as_ref()).unwrap();

            let last_updated = cprice.last_updated.unwrap_or(SystemTime::now());

            let datetime = DateTime::<Utc>::from(last_updated);
            let last_updated_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
          
            println!("last_updated::{:?}",last_updated_str);

            if last_updated.elapsed().unwrap().as_secs() > 3600 || force_refresh {

                let mut prices = get_price_of!(api_key, currency).await.unwrap();

               
                let serialized = serialize_and_set_time(&mut prices[0]);

                client.set (&price_id, bytes::Bytes::from(serialized) ).await.expect("Failed to set the serialized data");

            }
            else {

                println!("Stored priced of {} is {}", currency, cprice.price);
            }

        }
        ,
        None =>{

            let mut prices = get_price_of!(api_key, currency).await.unwrap();

            let serialized = serialize_and_set_time(&mut prices[0]);

            client.set (&price_id, bytes::Bytes::from(serialized) ).await.expect("Failed to set the serialized data");


        }
    }

}

fn serialize_and_set_time ( price : &mut CurrencyPrice) -> String {

    price.last_updated = Some(SystemTime::now());

    println!("Fetched remote price is ::{}", price.price);

    return serde_json::to_string(&price).unwrap();

}
