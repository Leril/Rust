pub struct Allergies{
    allergies: Vec<Allergen>,
}

#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        let mut allergies: Vec<Allergen> = Vec::new();
        let mut score = score;

        while score > 0{
            let power = Self::largest_power_of_two_divisor(score);
            let allergen = match power{
                0 => Allergen::Eggs,
                1 => Allergen::Peanuts,
                2 => Allergen::Shellfish,
                3 => Allergen::Strawberries,
                4 => Allergen::Tomatoes,
                5 => Allergen::Chocolate,
                6 => Allergen::Pollen,
                7 => Allergen::Cats,
                _ => Allergen::Eggs,
            };

            allergies.push(allergen);
            score /= 2;
        }

        Allergies{allergies}
    }

    fn largest_power_of_two_divisor(score: u32) -> u32{
        let mut res = 0;

        loop {
            if (2 as u32).pow(res) as u32 >= score{
                break;
            }
            res += 1;
        }

        res
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        for a in self.allergies{
            if a == *allergen{
                return true;
            }
        }
        false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergies.iter().clone().collect()
    }
}
