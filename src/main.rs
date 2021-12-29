mod stock_option;
use stock_option::{Stock, Simulation, std_deviation};
use serde::Deserialize;
use toml;
use rgsl::statistics::mean;
use std::fs::File;
use std::env;

extern crate time;
use time::PreciseTime;

#[derive(Deserialize)]
struct Config {
    filename: String,
    history: Option<usize>,
    time_steps: u32,
    iterations: Option<u64>,
    strike_price: f64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_filename = &args[1];
    let file = std::fs::read_to_string(config_filename).unwrap();
    let config: Config = toml::from_str(file.as_str()).unwrap();


    let i: Stock = Stock::from_csv(config.filename.as_str(), match config.history {Some(x) => x, _ => 20});
    let time_steps = config.time_steps;
    let data_points = match config.iterations{Some(x) => x, _ => 10_000_000};

    println!("loaded ticker with data: ");
    println!("----------------------------------------------------------");
    
    println!("volatility mean: {}\nvolatility stdev: {}\nstarting_price: {}", i.volatility, i.volatility_stdev, i.current_value);

    println!("---------------------------------------------------");
    println!("starting simulation:");
    
    println!("time-steps (amount of days/hours/..): {}\ndata-points (simulation precision): {}", config.time_steps, match config.iterations{Some(x) => x, _ => 10_000_000});
    println!("---------------------------------------------------");
    let sim = Simulation::new(i);
    let start = PreciseTime::now();
    let result2 = sim.rayon_multi_run(time_steps, data_points);
    let end = PreciseTime::now();


    println!("\n{} seconds for simulation run\n", start.to(end));
    println!("---------------------------------------------------");
    

    let sum: f64 = result2.iter().sum();
    let avg: f64 = sum/(result2.len() as f64);
    let std: f64 = std_deviation(&result2).unwrap();

    let strike_price = config.strike_price;
    let fair_price: f64 = Simulation::fair_price(config.strike_price, &result2);


    println!("fair price for option with strike_price {}: {}", strike_price,fair_price);
}
