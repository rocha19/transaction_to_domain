use std::sync::Arc;

use chrono::NaiveDateTime;

use crate::domain_model::{
    application::repositories::parked_car_repository::ParkedCarRepository,
    domain::models::period::Period,
};

#[allow(dead_code)]
pub struct CheckoutUseCase<D: ParkedCarRepository + Send + Sync> {
    working_hours: Period,
    parked_car: Arc<D>,
    open_hour: u32,
    close_hour: u32,
    parking_price: f64,
}

#[allow(dead_code)]
impl<D: ParkedCarRepository + Send + Sync> CheckoutUseCase<D> {
    pub fn new(working_hours: &Period, parked_car: Arc<D>) -> Self {
        let working_hours = working_hours.clone();
        Self {
            working_hours,
            parked_car,
            open_hour: 8,
            close_hour: 22,
            parking_price: 10.0,
        }
    }

    pub async fn perform(&self, plate: &str, checkout_date: NaiveDateTime) -> Result<f64, String> {
        if let Some(mut parked_car) = self.parked_car.get(plate).await {
            parked_car.checkout(checkout_date);
            self.parked_car.update(parked_car.clone()).await;
            Ok(parked_car.price.unwrap())
        } else {
            Err("Parked car not found".into())
        }
    }
}
