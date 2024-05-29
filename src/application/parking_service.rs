use std::{collections::HashMap, time::SystemTime};

use regex::Regex;

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct ParkingService {
    parked_cars: HashMap<String, (String, SystemTime)>,
}

#[allow(dead_code)]
impl ParkingService {
    pub fn new() -> Self {
        Self {
            parked_cars: HashMap::new(),
        }
    }

    pub async fn checkin(&mut self, plate: String, checkin_date: SystemTime) -> Result<(), String> {
        let plate_regex = Regex::new(r"^[A-Z]{3}\d{4}$").expect("Invalid regex pattern");

        match plate_regex.is_match(&plate) {
            true => {
                self.parked_cars
                    .insert(plate.clone(), (plate, checkin_date));
                Ok(())
            }
            false => Err("Invalid plate".to_string()),
        }
    }

    pub async fn checkout(
        &mut self,
        plate: &str,
        checkout_date: SystemTime,
    ) -> Result<f64, String> {
        let parked_car = self
            .parked_cars
            .remove(plate)
            .ok_or_else(|| "Car not found".to_string())?;
        let (_, checkin_date) = parked_car;

        let duration = checkout_date
            .duration_since(checkin_date)
            .map_err(|e| e.to_string())?;
        let duration_hours = duration.as_secs_f64() / 3600.0;
        let price = duration_hours * 10.0;
        Ok(price)
    }
}
