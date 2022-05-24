use std::fmt;
const HOURS_PER_DAY:i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = HOURS_PER_DAY * MINUTES_PER_HOUR;
#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * MINUTES_PER_HOUR) + minutes;
        let wraped_minutes =  (total_minutes % MINUTES_PER_DAY) + MINUTES_PER_DAY;
        let real_hours = wraped_minutes / MINUTES_PER_HOUR % HOURS_PER_DAY;
        let real_minutes = wraped_minutes % MINUTES_PER_HOUR;

        Self{
            hours: real_hours,
            minutes: real_minutes,
        }
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
