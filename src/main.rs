mod stock_option;
use stock_option::{Stock, Simulation};
use serde::Deserialize;
use toml;
use std::fs::File;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::path::Path;
use std::io::prelude::*;


extern crate time;
use time::PreciseTime;

#[derive(Deserialize)]
struct Config {
    history: Option<usize>,
    iterations: Option<u64>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("TODO USAGE");
        std::process::exit(1);
    }
    
    let ticker = &args[1];
    let strike_price = &args[2];
    let days_till_expiry = &args[3];
    let days_till_expiry: u32 = days_till_expiry.parse().unwrap();

    let strike_price: f64 = strike_price.parse().unwrap();

    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let year_ago = current_time - 31536000;
    let url_string: String = format!("https://query1.finance.yahoo.com/v7/finance/download/{}?period1={}&period2={}&interval=1d&events=history&includeAdjustedClose=true", ticker, year_ago, current_time);


    let filename = format!("./data/{}.csv", ticker);
    std::fs::create_dir_all("./data").expect("couldn't create data directory");


    let download = |filename: &String| {
        
            let mut file = match File::create(&filename) {
                Err(why) => panic!("couldn't create {}", why),
                Ok(file) => file,
            };
        
            let response = reqwest::blocking::get(url_string).unwrap().bytes().unwrap();
        
            file.write_all(&response).expect("Couldn't write downloaded data to file");
    };

    //let mut download_needed = false;
    if Path::new(&filename).exists() {
        let data = std::fs::metadata(&filename).unwrap();
        let last_acces = data.created().unwrap().duration_since(SystemTime::now()-Duration::from_secs(86400)).unwrap();
        if last_acces.as_secs() > 86400{
            std::fs::remove_file(&filename).expect("couldn't delete old data");
            download(&filename);
        }
    } else {
        download(&filename);
    }

    




    
    let config_file = std::fs::read_to_string("config.toml").unwrap();
    let config: Config = toml::from_str(config_file.as_str()).unwrap();


    let i: Stock = Stock::from_csv(&filename, match config.history {Some(x) => x, _ => 20});
    let time_steps = days_till_expiry;
    println!("timesteps: {}", time_steps);
    let data_points = match config.iterations{Some(x) => x, _ => 10_000_000};

    println!("loaded ticker with data: ");
    println!("----------------------------------------------------------");
    
    println!("volatility mean: {}\nvolatility stdev: {}\nstarting_price: {}", i.volatility, i.volatility_stdev, i.current_value);

    println!("---------------------------------------------------");
    println!("starting simulation:");
    
    println!("time-steps (amount of days/hours/..): {}\ndata-points (simulation precision): {}", days_till_expiry, match config.iterations{Some(x) => x, _ => 10_000_000});
    println!("---------------------------------------------------");
    let sim = Simulation::new(i);
    let start = PreciseTime::now();
    let result2 = sim.rayon_multi_run(time_steps, data_points);
    let end = PreciseTime::now();


    println!("\n{} seconds for simulation run\n", start.to(end));
    println!("---------------------------------------------------");
    

    let sum: f64 = result2.iter().sum();
    let avg: f64 = sum/(result2.len() as f64);

    let fair_price: f64 = Simulation::fair_price(strike_price, &result2);


    println!("average final value of stock: {}", avg);
    println!("fair price for option with strike_price {}: {}", strike_price,fair_price);
}
