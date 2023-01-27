mod data;
mod model;
mod ai;

use std::fs;
use ai::manager::get_prediction;
use crate::{data::{fetch, filter}, model::car_park::CarPark,};

fn main() {
    let file = fs::File::open("data.json").expect("no file");
    
    let master_data:Vec<CarPark>;
    let sample_data:Vec<CarPark>;

    master_data = fetch::fetch(file);
    sample_data = filter::hour(master_data.clone(), "08".to_string());

    let (lower,upper) = get_prediction(master_data, sample_data);
    println!("Range: {:?} - {:?} Parking Spaces", lower.round(),upper.round());
    println!("Median: {:?} Parking Spaces", ((lower + upper) / 2.0).floor());

}
