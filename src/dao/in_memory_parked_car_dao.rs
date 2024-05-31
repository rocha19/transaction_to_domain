use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::parked_car_dao::{ParkedCar, ParkedCarDAO};

#[derive(Default)]
pub struct ParkedCarDAOInMemory {
    parked_cars: Arc<Mutex<HashMap<String, ParkedCar>>>,
}

#[allow(dead_code)]
impl ParkedCarDAOInMemory {
    pub fn new() -> Self {
        Self {
            parked_cars: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ParkedCarDAO for ParkedCarDAOInMemory {
    async fn save(&self, parked_car: ParkedCar) {
        let mut parked_cars = self.parked_cars.lock().await;
        parked_cars.insert(parked_car.plate.clone(), parked_car);
    }

    async fn update(&self, parked_car: ParkedCar) {
        let mut parked_cars = self.parked_cars.lock().await;
        parked_cars.insert(parked_car.plate.clone(), parked_car);
    }

    async fn get(&self, plate: &str) -> Option<ParkedCar> {
        let parked_cars = self.parked_cars.lock().await;
        parked_cars.get(plate).cloned()
    }

    async fn delete(&self, plate: &str) {
        let mut parked_cars = self.parked_cars.lock().await;
        parked_cars.remove(plate);
    }
}
