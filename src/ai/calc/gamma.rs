use std::f64;

static G:i32 = 7;

pub fn gamma(mut z:f64) -> f64 {

    let p: Vec<f64> = [0.99999999999980993, 676.5203681218851, -1259.1392167224028,
    771.32342877765313, -176.61502916214059, 12.507343278686905,
    -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7].to_vec();

    if z < 0.5 {
        return f64::consts::PI / (f64::sin(f64::consts::PI * z) * gamma(1.0 - z));
    }
    z -= 1.0;
    let mut x = p[0];

    for i in 1..(G+2) {
        x += p[(i as usize)] / (z + (i as f64));
    }



    let t = z + (G as f64) + 0.5;
    return f64::sqrt(2.0 * f64::consts::PI) * (f64::powf(t, z + 0.5)) * f64::exp(-t) * x;
}