use std::f32;

static G:i32 = 7;

pub fn Gamma(mut z:f32) -> f32 {

    let P: Vec<f32> = [0.99999999999980993, 676.5203681218851, -1259.1392167224028,
    771.32342877765313, -176.61502916214059, 12.507343278686905,
    -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7].to_vec();

    if z < 0.5 {
        return f32::consts::PI / (f32::sin(f32::consts::PI * z) * Gamma(1.0 - z));
    }
    z -= 1.0;
    let mut x = P[0];

    for i in 1..(G+2) {
        x += P[(i as usize)] / (z + (i as f32));
    }



    let t = z + (G as f32) + 0.5;
    return f32::sqrt(2.0 * f32::consts::PI) * (f32::powf(t, z + 0.5)) * f32::exp(-t) * x;
}