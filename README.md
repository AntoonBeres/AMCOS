

# antoons-monte-carlo-options-sim
A high-peformance parallel monte-carlo simulation for estimating the fair value of options in finance.

Written in Rust ofcourse!


Work in progress. Currently has only minimal functionality. 


Historical market data is automatically downloaded from Yahoo Finance via their API and cached locally.

build instructions:

`cargo build --release`

Usage: 

`./amcos <TICKER SYMBOL> <STRIKE PRICE> <DAYS TILL EXPIRATION>`

example:

`./amcos AAPL 165 20`
