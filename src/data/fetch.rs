use std::fs::File;
use serde_json::Value;
use crate::model::car_park::CarPark;

/*
 Fetch & Parse Data, absolutely horrendous solution
*/
pub fn fetch(file:File) -> Vec<CarPark>{
    let mut car_parks:Vec<CarPark> = Vec::new();
    let dataset: Value = serde_json::from_reader(file).expect("JSON was not well-formatted");
    if let Value::Array(spaces) = &dataset["Availabilities"] {
        for park in spaces {
            car_parks.push(parse_park(park.clone()));
        }
    }
    return car_parks;
}

fn parse_park(data:Value) -> CarPark {
    let new = CarPark {
        id:data["Id"].to_string().replace("\"", ""),
        status:data["Status"].to_string().replace("\"", ""),
        moment:data["Moment"].to_string().replace("\"", ""),
        free:data["Free"].to_string().replace("\"", ""),
        car_park_id:data["CarParkId"].to_string().replace("\"", "")
    };
    return new;
}