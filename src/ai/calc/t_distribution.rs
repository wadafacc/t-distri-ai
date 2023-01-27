use crate::{ai::calc::gamma::gamma};
use std::f64;

pub fn t_score(sample_mean:f64, absolute_mean:f64, std_deviation:f64, count:i32) -> f64 {
    return (sample_mean-absolute_mean) / (std_deviation / f64::sqrt(count as f64));
}

pub fn t_distribution(count:i32, t_score:f64) -> f64 {
    let v:f64 = (count -1) as f64;

    let t_distri_val = 1.0 - (gamma(((v + 1.0) / 2.0) as f64) / (f64::sqrt((v*f64::consts::PI) as f64) * gamma((v / 2.0) as f64)) * f64::powf(1.0 + (f64::powf(t_score,2.0)) / v,(-v + 1.0) / 2.0));


    return t_distri_val;
}