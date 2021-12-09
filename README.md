# testrustasync

Just a playground of Rust to test async await to fetch the prices of cryptos from Nomics API and serialize each of them to
store on redis if it doesn't exist yet or its last fetched time is more than 1 hour or the --force-refresh is specified in
the cli argument  


Usage :

cargo run your-nomics-api-key ETH,SOL,BTC


## Force it to fetch from remote 
cargo run your-nomics-api-key ETH,SOL,BTC --force-refresh 


By Christopher Chee (2021)