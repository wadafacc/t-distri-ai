/*
in its own file, cuz why not.
*/

// calculate the deviation -> how accurate its prediction is
pub fn calc_std_deviation(data: Vec<i32>, count:i32) -> f64 {
    let mut sum = 0;
    for item in data {
        let val = (item - count) * (item - count);
        sum += val;
    }

    let variance = sum / count;
    return f64::sqrt(variance.into());
}   