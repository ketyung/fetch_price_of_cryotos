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


const API_KEY : &str = "replace-your-nomics-api-key-here";


#[macro_export]
macro_rules! get_price_of {
    
    ($a : expr ) => {
        
        get_price_of($a , String::from( "USD" ) )
    };
}


pub async fn get_price_of(currency : String, convert_to : String) -> Result<Vec<CurrencyPrice>, Error>  {

    let request_url = format!(
        "https://api.nomics.com/v1/currencies/ticker?key={key}&ids={ids}&convert={convert}&interval={interval}", 

        key = API_KEY, 

        ids = currency,

        convert = convert_to,

        interval = "1h",
    );
    
    println!("{}", request_url);


    let response = reqwest::get(&request_url).await?;

    let prices : Vec<CurrencyPrice>  = response.json().await?;

    Ok(prices)
}



pub async fn index_price_for (symbol : &str, client : &mut client::Client) {

    let price_id = format! ("{}{}{}", PRICE_PREFIX , symbol, "USD") ;

    let res = client.get( &price_id ).await.unwrap();

    match res {

        Some(s) =>{
   
            let cprice : CurrencyPrice = serde_json::from_slice(s.as_ref()).unwrap();

            let last_updated = cprice.last_updated.unwrap_or(SystemTime::now());

            let datetime = DateTime::<Utc>::from(last_updated);
            let last_updated_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
          
            println!("last_updated::{:?}",last_updated_str);

            if last_updated.elapsed().unwrap().as_secs() > 3600 {

                let mut prices = get_price_of!(String::from(symbol)).await.unwrap();

               
                let serialized = serialize_and_set_time(&mut prices[0]);

                client.set (&price_id, bytes::Bytes::from(serialized) ).await.expect("Failed to set the serialized data");

            }
            else {

                println!("Stored priced is {}", cprice.price);
            }

        }
        ,
        None =>{

            let mut prices = get_price_of!(String::from(symbol)).await.unwrap();

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
