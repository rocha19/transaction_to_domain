// dao.rs
use async_trait::async_trait;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ParkedCar {
    pub plate: String,
    pub checkin_date: chrono::NaiveDateTime,
    pub checkout_date: Option<chrono::NaiveDateTime>,
    pub duration: Option<f64>,
    pub price: Option<f64>,
}

#[async_trait]
pub trait ParkedCarDAO {
    async fn save(&self, parked_car: ParkedCar);
    async fn update(&self, parked_car: ParkedCar);
    async fn get(&self, plate: &str) -> Option<ParkedCar>;
    async fn delete(&self, plate: &str);
}
