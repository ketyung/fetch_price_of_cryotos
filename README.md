# testrustasync

Just a playground of Rust to test async await to fetch the price of a crypto from Nomics API and serialize it to
store on redis if it doesn't exist yet or its last fetched time is more than 1 hour 


Usage :

cargo run your-nomics-api-key ETH,SOL,BTC


## Force it to fetch from remote 
cargo run your-nomics-api-key ETH,SOL,BTC force-refresh 

