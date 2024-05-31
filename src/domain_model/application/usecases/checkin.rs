use std::sync::Arc;

use crate::domain_model::{
    application::repositories::parked_car_repository::ParkedCarRepository,
    domain::{entities::parked_car::ParkedCar, models::period::Period},
};
use chrono::{NaiveDateTime, Timelike};

#[allow(dead_code)]
pub struct CheckinUseCase<D: ParkedCarRepository + Send + Sync> {
    working_hours: Period,
    parked_car: Arc<D>,
    open_hour: u32,
    close_hour: u32,
    parking_price: f64,
}

#[allow(dead_code)]
impl<D: ParkedCarRepository + Send + Sync> CheckinUseCase<D> {
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

    pub async fn perform(&self, plate: &str, checkin_date: NaiveDateTime) -> Result<(), String> {
        let checkin_hour = checkin_date.hour();
        if checkin_hour < self.open_hour || checkin_hour >= self.close_hour {
            return Err("Invalid checkin date".into());
        }

        let parked_car = ParkedCar::new(plate.into(), checkin_date)?;
        self.parked_car.save(parked_car).await;
        Ok(())
    }
}
