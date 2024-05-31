use crate::dao::parked_car_dao::ParkedCar;
use chrono::{NaiveDateTime, Timelike};
use regex::Regex;

use super::parked_car_dao::ParkedCarDAO;

#[allow(dead_code)]
pub struct ParkingServiceDAO<D: ParkedCarDAO + Send + Sync> {
    parked_car_dao: D,
    open_hour: u32,
    close_hour: u32,
    parking_price: f64,
}

#[allow(dead_code)]
impl<D: ParkedCarDAO + Send + Sync> ParkingServiceDAO<D> {
    pub fn new(parked_car_dao: D) -> Self {
        Self {
            parked_car_dao,
            open_hour: 8,
            close_hour: 22,
            parking_price: 10.0,
        }
    }

    pub async fn checkin(&self, plate: &str, checkin_date: NaiveDateTime) -> Result<(), String> {
        let plate_regex = Regex::new(r"^[A-Z]{3}[0-9]{4}$").unwrap();
        match (checkin_date.hour(), plate) {
            (hour, _) if hour < self.open_hour || hour >= self.close_hour => {
                Err("Parking lot is closed".into())
            }
            (_, plate) if !plate_regex.is_match(plate) => Err("Invalid plate".into()),
            _ => {
                let parked_car = ParkedCar {
                    plate: plate.into(),
                    checkin_date,
                    checkout_date: None,
                    duration: None,
                    price: None,
                };
                self.parked_car_dao.save(parked_car).await;
                Ok(())
            }
        }
    }

    pub async fn checkout(&self, plate: &str, checkout_date: NaiveDateTime) -> Result<f64, String> {
        if let Some(mut parked_car) = self.parked_car_dao.get(plate).await {
            parked_car.checkout_date = Some(checkout_date);
            let duration = (checkout_date - parked_car.checkin_date).num_hours() as f64;
            parked_car.duration = Some(duration);
            parked_car.price = Some(duration * 10.0);
            self.parked_car_dao.update(parked_car.clone()).await;
            Ok(parked_car.price.unwrap())
        } else {
            Err("Parked car not found".into())
        }
    }
}
