// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut res: f64 = 0.0;
    if speed > 0 && speed < 5{
        res = (speed as f64) * (221 as f64)
    }else if speed >4 && speed < 9{
        res = (speed as f64) * (221 as f64)* 0.9
    }else if speed == 9 || speed == 10{
        res = (speed as f64) * (221 as f64)* 0.77
    }

    res
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let rate_per_hour = production_rate_per_hour(speed);
    (rate_per_hour / (60 as f64)) as u32
}