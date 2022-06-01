#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default)]
pub struct BowlingGame {
    second_throw: bool,
    throws: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame{
            second_throw: false,
            throws: vec![]
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 || (self.second_throw && pins + self.throws.last().unwrap() > (10 as u16)){
            return Err(Error::NotEnoughPinsLeft);
        }else if self.score().is_some(){
            return Err(Error::GameComplete);
        }

        self.throws.push(pins);
        self.second_throw = match pins{
            10 => false,
            _ => !self.second_throw,
        };

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut sum = 0;
        let mut index = 0;

        for i in 0..10{
            if let (Some(&t1), Some(&t2)) = (&self.throws.get(index), self.throws.get(index + 1)){
                sum += t1 + t2;

                if t1 == 10 || t1 + t2 == 10{
                    if let Some(&t3) = &self.throws.get(index + 2){
                        sum += t3;
                    }else {
                        return None;
                    }
                }

                index += if t1 == 10 {1} else {2};
            }else { return None }
        }
        Some(sum)
    }
}