use serde::{Serialize, Deserialize};
use reqwest::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyPrice {

    pub currency : String, 

    pub id : String, 

    pub price : String,

    pub name : String, 
}



pub async fn get_price_of(currency : String, convert_to : String) -> Result<Vec<CurrencyPrice>, Error>  {


    let request_url = format!(
        "https://api.nomics.com/v1/currencies/ticker?key={key}&ids={ids}&convert={convert}&interval={interval}", 

        key = "8e5c68f6963ab562148b80ddf121fc73b00808d0", 

        ids = currency,

        convert = convert_to,

        interval = "1h",
    );
    
    println!("{}", request_url);


    let response = reqwest::get(&request_url).await?;

    let prices : Vec<CurrencyPrice>  = response.json().await?;

    Ok(prices)
}