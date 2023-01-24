mod data;
mod model;
mod ai;

use std::fs::{self, File};
use serde_json::Value;
use crate::{data::{fetch, filter}, model::car_park::CarPark,};

fn main() {
    let file = fs::File::open("data.json").expect("no file");
    
    let master_data:Vec<CarPark>;
    let sample_data:Vec<CarPark>;

    master_data = fetch::fetch(file);
    sample_data = filter::hour(master_data, "08".to_string());    
}
