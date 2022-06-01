use std::collections::HashSet;
use std::fmt::{Display, Formatter, Write};
use crate::random::random_range;

pub type Position = (usize, usize);

pub enum OpenResult{
    Mine,
    NoMine(u8)
}

#[derive(Debug)]
pub struct Minesweeper{
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flagged_fields: HashSet<Position>,
    lost: bool,
}

impl Minesweeper{
    pub fn new(width: usize, height: usize, mine_count: usize) -> Minesweeper{
        Minesweeper{
            width,
            height,
            open_fields: HashSet::new(),
            mines: Minesweeper::generate_mines(width, height, mine_count),
            flagged_fields: HashSet::new(),
            lost: false,
        }
    }

    pub fn open(&mut self, position: Position) -> Option<OpenResult>{
        if self.open_fields.contains(&position){
            let mine_count = self.neighbour_mines(position);
            let flag_count = self.neighbours_iter(position)
                .filter(|neighbour| self.flagged_fields
                .contains(neighbour))
                .count() as u8;

            if mine_count == flag_count{
                for neighbor in self.neighbours_iter(position){
                    if !self.flagged_fields.contains(&neighbor)  && !self.open_fields.contains(&position){
                        self.open(neighbor);
                    }
                }
            }

            return None;
        }

        if self.flagged_fields.contains(&position) || self.lost {
            return None;
        }

        self.open_fields.insert(position);

        let is_mine = self.mines.contains(&position);

        if is_mine {
            self.lost = true;
            Some(OpenResult::Mine)
        } else {
            let mine_count = self.neighbour_mines(position);

            if mine_count == 0{
                for neighbor in self.neighbours_iter(position){
                    if !self.open_fields.contains(&neighbor) {
                        self.open(neighbor);
                    }
                }
            }

            Some(OpenResult::NoMine(mine_count))
        }
    }

    pub fn toggle_flag(&mut self, pos: Position){
        if self.open_fields.contains(&pos) || self.lost{
            return;
        }

        if self.flagged_fields.contains(&pos){
            self.flagged_fields.remove(&pos);
        }else {
            self.flagged_fields.insert(pos);
        }
    }

    fn neighbours_iter(&self, (x,y): Position) -> impl Iterator<Item = Position>{
        let w = self.width;
        let h = self.height;

        (x.max(1) - 1..=(x + 1).min(w - 1))
            .flat_map(move |i|{
                (y.max(1) - 1..=(y+1).min(h- 1)).map(move |j| (i,j))
            }).filter(move |&pos| pos != (x,y))
    }

    fn neighbour_mines(&self, pos:Position) -> u8{
        self.neighbours_iter(pos)
            .filter(|pos| self.mines.contains(pos))
            .count() as u8
    }

    fn generate_mines(width: usize, height: usize, mine_count: usize) -> HashSet<Position>{
        let mut mines = HashSet::new();

        while mines.len() < mine_count{
            mines.insert((random_range(0, width), random_range(0, height)));
        }
        mines
    }
}

impl Display for Minesweeper{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.width{
            for  y in  0..self.height{
                let pos = (x,y);

                if !self.open_fields.contains(&pos){
                    if self.lost && self.mines.contains(&pos){
                        f.write_str(" 💣 ")?;
                    }else if self.flagged_fields.contains(&pos){
                        f.write_str("  🚩 ")?;
                    }else {
                        f.write_str(" 🟧  ")?;
                    }
                }else if self.mines.contains(&pos){
                    f.write_str(" 💣 ")?;
                }else{
                    let mine_count = self.neighbour_mines(pos);

                    if mine_count > 0{
                        write!(f, " {} ", self.neighbour_mines(pos))?;
                    }else {
                        f.write_str(" ⬜ ")?;
                    }
                }
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use crate::minesweeper::Minesweeper;

    #[test]
    fn test(){
        let mut ms = Minesweeper::new(10, 10, 10);
        ms.open((5,5));
        ms.toggle_flag((3,3));

        println!("{}", ms);
    }
}