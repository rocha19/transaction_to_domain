use async_trait::async_trait;

use crate::domain_model::domain::entities::parked_car::ParkedCar;

#[async_trait]
pub trait ParkedCarRepository {
    async fn save(&self, parked_car: ParkedCar);
    async fn update(&self, parked_car: ParkedCar);
    async fn get(&self, plate: &str) -> Option<ParkedCar>;
    async fn delete(&self, plate: &str);
}
