use std::{cell::RefCell, time::SystemTime};

#[allow(dead_code)]
pub trait Clock {
    fn get_current_date(&self) -> SystemTime;
}

#[allow(dead_code)]
struct ClockFake {
    current_date: RefCell<SystemTime>,
}

#[allow(dead_code)]
impl ClockFake {
    fn set_current_date(&self, date: SystemTime) {
        *self.current_date.borrow_mut() = date;
    }
}

#[allow(dead_code)]
impl Clock for ClockFake {
    fn get_current_date(&self) -> SystemTime {
        *self.current_date.borrow()
    }
}
