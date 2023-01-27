use crate::model::car_park::CarPark;

pub fn hour(data:Vec<CarPark>, hour: String) -> Vec<CarPark> {
    let mut samples:Vec<CarPark> = Vec::new();

    for e in data {
        let (_date,time) = e.moment.split_once(" ").unwrap();
        if time.split(":").next().unwrap().contains(&hour) {
            samples.push(e);
        }
    }

    return samples;
}

pub fn free(data:Vec<CarPark>) -> Vec<i32> {
    let mut values:Vec<i32> = Vec::new();
    for x in data {
        values.push(x.free.parse::<i32>().unwrap());
    }
    return values;
}