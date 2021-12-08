use serde::{Serialize, Deserialize};
use reqwest::Error;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyPrice {

    pub currency : String, 

    pub id : String, 

    pub price : String,

    pub name : String, 

    pub last_updated : Option<SystemTime>,
}


pub const PRICE_PREFIX : &str = "nomics::price::";


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