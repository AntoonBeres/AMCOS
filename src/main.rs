mod stock_option;
use stock_option::{StockOption, Stock};
use stock_option::Simulation;
use stock_option::std_deviation;
use std::fs::File;
use csv;

use rgsl::statistics::mean;

extern crate time;
use time::PreciseTime;

fn main() {
    
    let i: Stock = Stock::from_csv("RKLB.csv", 20);

    println!("volatility mean: {}\n volatility stdev: {}", i.volatility, i.volatility_stdev);


    let sim = Simulation::new(i);

    let start = PreciseTime::now();
    let result2 = sim.rayon_multi_run(60, 1_00_000);
    let end = PreciseTime::now();


    println!("\n{} seconds for rayon\n", start.to(end));

    

    let sum: f64 = result2.iter().sum();
    let avg: f64 = sum/(result2.len() as f64);
    let std: f64 = std_deviation(&result2).unwrap();

    let fair_price: f64 = Simulation::fair_price(14.0, &result2);
    /*for i in result{
        println!("{}", i);
    }*/
    println!("average2: {}", avg);
    println!("std2: {}", std);
    println!("len2: {}", result2.len());

    println!("fair price: {}", fair_price);
}
