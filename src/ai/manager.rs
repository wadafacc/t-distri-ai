use crate::{model::car_park::CarPark, data::filter};
use crate::{ai::calc::std_dev};

pub fn get_prediction(master_data:Vec<CarPark>, sample_data: Vec<CarPark>) -> Vec<f32> {
    let mut bounds:Vec<f32> = Vec::new();

    // check if there is data available for the selected timeframe
    if sample_data.len() == 0 {
        bounds.push(0.0);
        bounds.push(82.0);
        return bounds;
    }

    let filtered_samples = filter::free(sample_data);
    let sample_mean = ((filtered_samples.iter().sum::<i32>()) / (filtered_samples.len() as i32)) as f32;
    let count = filtered_samples.clone().len() as i32;
    
    let filtered_absolutes:Vec<i32> = filter::free(master_data);
    let absolute_mean = ((filtered_absolutes.iter().sum::<i32>()) / (filtered_absolutes.len() as i32)) as f32;


    let std_deviation = std_dev::calc_std_deviation(filtered_samples, count);


    return bounds;
}