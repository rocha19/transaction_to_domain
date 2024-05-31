use chrono::NaiveDateTime;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Period {
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
}

#[allow(dead_code)]
impl Period {
    pub fn new(start_date: NaiveDateTime, end_date: NaiveDateTime) -> Result<Self, String> {
        if start_date > end_date {
            return Err("Invalid period".into());
        }
        Ok(Period {
            start_date,
            end_date,
        })
    }

    pub fn start_date(&self) -> NaiveDateTime {
        self.start_date
    }

    pub fn end_date(&self) -> NaiveDateTime {
        self.end_date
    }

    pub fn get_duration_in_hours(&self) -> f64 {
        (self.end_date - self.start_date).num_hours() as f64
    }

    pub fn is_out_of_period(&self, date: NaiveDateTime) -> bool {
        date < self.start_date || date > self.end_date
    }
}
