// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.


pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health > 0{
            return Option::None
        }

        let m = if self.level > 9{
            Option::Some(100)
        }else { Option::None };

        let p = Player{
            health: 100,
            mana: m,
            level: self.level
        };
        Option::Some(p)
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        let mut mana = match self.mana {
            None => {0}
            Some(x) => {x}
        };

        return if self.level < 10 {
            if self.health > mana_cost{
                self.health -= mana_cost;
            }else{
                self.health = 0;
            }
            0
        } else if mana > mana_cost {
            mana -= mana_cost;
            self.mana = Option::Some(mana);
            mana_cost * 2
        } else {
            0
        }
    }
}
