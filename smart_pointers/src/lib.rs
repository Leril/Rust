pub trait Messenger{
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger>{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl <'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T>{
        LimitTracker{
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize){
        self.value = value;

        let percentage = self.value as f64/self.max as f64;

        if percentage >= 1 as f64 {
            self.messenger.send("Error you used too much");
        }else if percentage >= 0.9 {
            self.messenger.send("Urgent warning");
        }else if percentage >= 0.7{
            self.messenger.send("warning");
        }
    }
}