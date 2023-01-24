pub fn t_score(sample_mean:f32, absolute_mean:f32, std_deviation:f32, count:i32) -> f32 {
    return (sample_mean-absolute_mean) / (std_deviation / f32::sqrt(count as f32));
}

pub fn t_distribution(sample_mean:f32, absolute_mean:f32, std_deviation:f32, count:i32) -> f32 {
    return 0.0;
}