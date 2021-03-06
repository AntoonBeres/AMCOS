use rand::prelude::*;
use rand_distr::{Normal, Distribution};
use serde::Deserialize;
use std::fs::File;
use std::error::Error;

use rayon::prelude::*;


fn mean(data: &Vec<f64>) -> Option<f64> {
    let sum = data.iter().sum::<f64>() as f64;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

pub fn std_deviation(data: &Vec<f64>) -> Option<f64> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f64);

                diff * diff
            }).sum::<f64>() / count as f64;

            Some(variance.sqrt())
        },
        _ => None
    }
}

#[allow(dead_code)]
pub enum OptionType {
    Call,
    Put
}

#[derive(Debug, Deserialize)]
struct StockRecord {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Open")]
    open: f64,
    #[serde(rename = "High")]
    high: f64,
    #[serde(rename = "Low")]
    low: f64,
    #[serde(rename = "Close")]
    close: f64,
    #[serde(rename = "Adj Close")]
    adj_close: f64,
    #[serde(rename = "Volume")]
    volume: u64
}
type StockDataSet = Vec<StockRecord>;

fn read_stock_data_set_from_csv(file_path: &str) -> Result<StockDataSet,Box<dyn Error>> {
    let mut result_data: StockDataSet = Vec::new();
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);


    for result in rdr.deserialize() {
        let record: StockRecord = result?;
        result_data.push(record);
    }

    Ok(result_data)
}


pub struct Stock {
    pub current_value: f64,
    basline_growth: f64,
    pub volatility: f64,
    pub volatility_stdev: f64,

    #[allow(dead_code)]
    price_points_per_day: u64
}

impl Stock {
    pub fn from_csv(file_path: &str, n_datapoints: usize) -> Stock {
        let dataset = read_stock_data_set_from_csv(file_path).unwrap();
        let current_value = dataset.last().unwrap().close;
        
        let mut price_points: Vec<f64> = Vec::new();
        for record in &dataset[dataset.len()-n_datapoints.. ] {
            price_points.push(record.close);
        }

        /*let mut price_point_change: Vec<f64> = Vec::new();*/
        let mut price_point_change_abs: Vec<f64> = Vec::new();
        
        for i in 0..(price_points.len()-1){
            price_point_change_abs.push( ((price_points[i]-price_points[i+1]).abs())/price_points[i]  );
        }

        
        /*
        let total_interest_rate = ((price_points.last().unwrap())/(price_points.first().unwrap()));
        println!("total rate {}", total_interest_rate-1.0);
        let basline_growth = (f64::powf(total_interest_rate.abs(), (1.0/(price_points.len() as f64)))) - 1.0;
        println!("total rate {}", basline_growth);*/

        let mean_volatility = mean(&price_point_change_abs).unwrap();
        let volatility_stdev = std_deviation(&price_point_change_abs).unwrap();

        Stock {
            current_value: current_value,
            basline_growth: 0.0,
            volatility: mean_volatility,
            volatility_stdev: volatility_stdev,
            price_points_per_day: 1
        }
    }

}


pub struct Simulation {
    asset: Stock,
}


impl Simulation {
    pub fn new(asset: Stock) -> Simulation {
        Simulation{
            asset: asset,
        }
    }
    pub fn rayon_multi_run(&self, n_steps: u32, n_runs: u64) -> Vec<f64> {
        let startval = self.asset.current_value;
        let mut result:Vec<f64> = vec![startval; n_runs as usize];
        let volatility = self.asset.volatility;
        let stdev_volatility = self.asset.volatility_stdev;

        //let risk_free_rate = self.asset.basline_growth;
        
        result.par_iter_mut().for_each(|x| {
            let mut rng = thread_rng();
            let mut i: f64 = match rng.gen::<bool>() {
                true => 1.0,
                false => -1.0
            };
            let normal_dist: Normal<f64> = Normal::new(volatility, stdev_volatility).unwrap(); 
            let mut norm = i* startval * normal_dist.sample(&mut rng);
            for _i in 0..n_steps {
                *x = *x + norm /*+ (*x * risk_free_rate)*/;
                norm = i * *x * normal_dist.sample(&mut rng);
                i = match rng.gen::<bool>() {
                    true => 1.0,
                    false => -1.0
                };
            }
        });
        result
    }

    //TODO: DEAL WITH FLOATING POINT ROUNDING!
    pub fn fair_price(strik_price: f64, simdata: &Vec<f64>) -> f64 {
        let mut total: f64 = 0.;
        for &i in simdata.iter() {
            let mut dif = i - strik_price;
            if dif < 0. {dif = 0.;}
            total += dif; 
        }
        total = total/ (simdata.len() as f64);
        total
    }
}