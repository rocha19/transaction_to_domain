use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::domain_model::{
    application::repositories::parked_car_repository::ParkedCarRepository,
    domain::entities::parked_car::ParkedCar,
};

#[derive(Default)]
pub struct InMemoryParkedCarRepository {
    parked_cars: Mutex<HashMap<String, ParkedCar>>,
}

#[async_trait]
impl ParkedCarRepository for InMemoryParkedCarRepository {
    async fn save(&self, parked_car: ParkedCar) {
        let mut lock = self.parked_cars.lock().await;
        lock.insert(parked_car.plate.value().to_string(), parked_car);
    }

    async fn update(&self, parked_car: ParkedCar) {
        let mut lock = self.parked_cars.lock().await;
        lock.insert(parked_car.plate.value().to_string(), parked_car);
    }

    async fn get(&self, plate: &str) -> Option<ParkedCar> {
        let lock = self.parked_cars.lock().await;
        lock.get(plate).cloned()
    }

    async fn delete(&self, plate: &str) {
        let mut lock = self.parked_cars.lock().await;
        lock.remove(plate);
    }
}
