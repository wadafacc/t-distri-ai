use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CarPark {
    pub id: String,
    pub status:String,
    pub free:String,
    pub moment:String,
    pub car_park_id:String
}