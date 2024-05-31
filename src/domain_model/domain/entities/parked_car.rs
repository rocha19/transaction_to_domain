use crate::domain_model::domain::models::{period::Period, plate::Plate};
use chrono::NaiveDateTime;

#[derive(Clone)]
pub struct ParkedCar {
    pub price_per_hour: f64,
    pub plate: Plate,
    pub checkin_date: NaiveDateTime,
    pub checkout_date: Option<NaiveDateTime>,
    pub period: Option<Period>,
    pub price: Option<f64>,
}

#[allow(dead_code)]
impl ParkedCar {
    pub fn new(plate: String, checkin_date: NaiveDateTime) -> Result<Self, String> {
        let plate = Plate::new(&plate)?;
        Ok(Self {
            price_per_hour: 10.0,
            plate,
            checkin_date,
            checkout_date: None,
            period: None,
            price: None,
        })
    }

    pub fn checkout(&mut self, checkout_date: NaiveDateTime) {
        self.checkout_date = Some(checkout_date);
        self.period = Some(Period::new(self.checkin_date, checkout_date).unwrap());
        self.price =
            Some(self.period.as_ref().unwrap().get_duration_in_hours() * self.price_per_hour);
    }

    pub fn calculate_price(&self) -> f64 {
        if let Some(period) = &self.period {
            period.get_duration_in_hours() * self.price_per_hour
        } else {
            panic!("Car not parked yet");
        }
    }
}
