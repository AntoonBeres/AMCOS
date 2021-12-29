# antoons-monte-carlo-options-sim
A high-peformance parallel monte-carlo simulation for estimating the fair value of options in finance.

Written in Rust ofcourse!


Work in progress. Currently has only minimal functionality. 


The CSV-files for getting the historical data are downloaded from yahoo-finance (Historical-data -> Download).
In the future an API might be used for more precise datapoints. As this happens, the interface will change.

build instructions:

`cargo build --release`

Usage: 

`./AMCOS <CONFIG_FILE>`

example:

`./AMCOS config.toml`

