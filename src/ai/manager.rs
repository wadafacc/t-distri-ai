use crate::{model::car_park::CarPark, data::filter, ai::calc::t_distribution};
use crate::{ai::calc::std_dev};



pub fn get_prediction(master_data:Vec<CarPark>, sample_data: Vec<CarPark>) -> (f64, f64) {
    // check if there is data available for the selected timeframe
    if sample_data.len() == 0 {
        return (0.0, 82.0);
    }

    let filtered_samples = filter::free(sample_data);
    let sample_mean = ((filtered_samples.iter().sum::<i32>()) / (filtered_samples.len() as i32)) as f64;
    let count = filtered_samples.clone().len() as i32;
    
    let filtered_absolutes:Vec<i32> = filter::free(master_data);
    let absolute_mean = ((filtered_absolutes.iter().sum::<i32>()) / (filtered_absolutes.len() as i32)) as f64;


    let std_deviation = std_dev::calc_std_deviation(filtered_samples, count);

    let (lower, upper) = calc_bounds(std_deviation, sample_mean, absolute_mean, count);
    return (lower, upper);
}

pub fn calc_bounds(std_dev:f64, sample_mean:f64, absolute_mean:f64, count:i32) -> (f64, f64) {
    let t_score = t_distribution::t_score(sample_mean, absolute_mean, std_dev, count);

    let crit_val = t_distribution::t_distribution(count, t_score);

    let lower = sample_mean - ((crit_val * std_dev) / f64::sqrt(count as f64));
    let upper = sample_mean + ((crit_val * std_dev) / f64::sqrt(count as f64));
    
    return (lower, upper);
}