use std::time::Duration;
use chip8_base::{Display, Interpreter, Keys, Pixel};
use chip8_base::Pixel::Black;
use rand::random;

use crate::interpreter::instruction::{decode, InstructionTypes};
use crate::interpreter::pixel::PixelIterator;

mod instruction;
mod pixel;
mod font;

pub struct VM{
    memory: [u8; 4096],
    registers: [u8; 16],
    display: [[Pixel; 64]; 32],
    speed: Duration,
    program_counter: u16,
    index: u16,
    delay_timer: u8,
    sound_timer: u8,
    ticker: u32,
    max_ticks:u32,
    stack: Vec<u16>
}

impl Interpreter for VM{
    fn step(&mut self, keys: &Keys) -> Option<Display> {
        let next_instruction = self.fetch();
        let decoded = decode(next_instruction);
        let updated_display = self.execute(keys, decoded);

        self.ticker += 1;

        if self.ticker == self.max_ticks{
            self.ticker = 0;
            self.delay_timer = self.delay_timer.saturating_sub(1);
            self.sound_timer = self.sound_timer.saturating_sub(1);
        }

        updated_display
    }

    fn speed(&self) -> Duration {
        self.speed
    }

    fn buzzer_active(&self) -> bool {
        self.sound_timer != 0
    }
}

impl VM {
    pub fn new(speed: Duration) -> VM{
        let mut memory = [0_u8; 4096];

        memory[0x50..(0x50 + 80)].copy_from_slice(&font::FONT);

        VM{
            memory: [0; 4096],
            registers: [0; 16],
            display: [[Pixel::default(); 64]; 32],
            speed,
            program_counter: 0,
            index: 0,
            delay_timer: 0,
            sound_timer: 0,
            ticker: 0,
            max_ticks: (speed.as_secs_f64() / 60 as f64).round() as u32,
            stack: Vec::new()
        }
    }

    pub fn load(mut self, filename: &str) -> std::io::Result<Self>{
        let program = std::fs::read(filename)?;
        self.memory[0x200..(0x200 + program.len())].copy_from_slice(&program);
        self.program_counter = 0x200;
        Ok(self)
    }

    fn fetch(&mut self) -> u16{
        let next = u16::from_be_bytes([
            self.memory[self.program_counter as usize],
            self.memory[(self.program_counter + 1) as usize]
        ]);

        if self.program_counter == 4096{
            self.program_counter = 0;
        }

        self.advance_program_counter();
        next
    }

    fn advance_program_counter(&mut self){
        self.program_counter += 2;
        self.program_counter &= 0xfff;
    }

    fn execute(&mut self,keys: &Keys, instruction :InstructionTypes) -> Option<Display>{
        let vf: usize = 0xf;

        if instruction != InstructionTypes::SysOld{
            dbg!("{}", &instruction);
        }

        match instruction {
            InstructionTypes::SysOld => (),
            InstructionTypes::Clear => {
                self.display = [[Black; 64]; 32];
                return Some(self.display);
            }
            InstructionTypes::Return => {
                self.program_counter = self.stack.pop().unwrap_or(0);
            }
            InstructionTypes::Jump(addr) => {
                self.program_counter = addr;
            }
            InstructionTypes::Call(addr) => {
                self.stack.push(self.program_counter);
                self.program_counter = addr;
            }
            InstructionTypes::SkipIfKK(x, kk) => {
                if self.registers[x as usize] == kk{
                    self.advance_program_counter();
                }
            }
            InstructionTypes::SkipIfNotKK(x, kk) => {
                if self.registers[x as usize] != kk{
                    self.advance_program_counter();
                }
            }
            InstructionTypes::SkipIfCompare(x, y) => {
                if self.registers[x as usize] == self.registers[y as usize]{
                    self.advance_program_counter();
                }
            }
            InstructionTypes::SetKK(x, kk) => {
                self.registers[x as usize] = kk;
            }
            InstructionTypes::AddKK(x, kk) => {
                self.registers[x as usize] = self.registers[x as usize].wrapping_add(kk);
            }
            InstructionTypes::SetTo(x, y) => {
                self.registers[x as usize] = self.registers[y as usize];
            }
            InstructionTypes::BitOr(x, y) => {
                self.registers[x as usize] |= self.registers[y as usize];
            }
            InstructionTypes::BitAnd(x, y) => {
                self.registers[x as usize] &= self.registers[y as usize];
            }
            InstructionTypes::BitXor(x, y) => {
                self.registers[x as usize] ^= self.registers[y as usize];
            }
            InstructionTypes::BitAdd(x, y) => {
                let (res, rem) =
                    self.registers[x as usize].overflowing_add(self.registers[y as usize]);
                self.registers[x as usize] = res;
                self.registers[vf] = rem.into();
            }
            InstructionTypes::BitSub(x, y) => {
                let (res, rem) =
                    self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
                self.registers[x as usize] = res;
                self.registers[vf] = rem.into();
            }
            InstructionTypes::BitShr(x, _) => {
                self.registers[vf] = 1 & self.registers[x as usize];
                self.registers[x as usize] >>= 1;
            }
            InstructionTypes::BitSubN(x, y) => {
                let (res, rem) =
                    self.registers[y as usize].overflowing_add(self.registers[x as usize]);
                self.registers[x as usize] = res;
                self.registers[vf] = rem.into();
            }
            InstructionTypes::BitSHL(x, _) => {
                self.registers[vf] = 0x80 & &self.registers[x as usize];
                self.registers[x as usize] <<= 1;
            }
            InstructionTypes::SkipNextIfNotEqual(x, y) => {
                if self.registers[x as usize] != self.registers[y as usize]{
                    self.advance_program_counter();
                }
            }
            InstructionTypes::SetN(nnn) => {
                self.index = nnn;
            }
            InstructionTypes::JumpN(nnn) => {
                self.program_counter = (nnn + self.registers[0] as u16) & 0xfff;
            }
            InstructionTypes::RandomKK(x, kk) => {
                self.registers[x as usize] = random::<u8>() & kk;
            }
            InstructionTypes::Draw(x, y, n) => {
                let range = (self.index as usize)..((self.index + n as u16) as usize);
                let sprite = &self.memory[range];

                let x_pos = self.registers[x as usize] % 64;
                let y_pos = self.registers[y as usize] % 32;
                self.registers[0xf] = 0;

                for(i, row) in sprite.iter().enumerate(){
                    if y_pos + i as u8 > 31{
                        break;
                    }

                    for(j, pixel) in (0..8).zip(PixelIterator::new(row)){
                        if x_pos + j as u8 > 63{
                            break;
                        }

                        let display_px = &mut self.display[(y_pos as usize + i)][x_pos as usize + j];

                        if (*display_px & pixel).into() {
                            self.registers[0xf] = 1;
                        }

                        *display_px ^= pixel;
                    }
                }
                return Some(self.display);
            }
            InstructionTypes::SkipIfPressed(x) => {
                if keys[self.registers[x as usize] as usize]{
                    self.advance_program_counter();
                }
            }
            InstructionTypes::SkipIfNotPressed(x) => {
                if !keys[self.registers[x as usize] as usize]{
                    self.advance_program_counter();
                }
            }
            InstructionTypes::GetTimer(x) => {
                self.registers[x as usize] = self.delay_timer;
            }
            InstructionTypes::WaitFor(x) => {
                if keys.iter().all(|k| !*k){
                    self.program_counter += 2;
                }else{
                    self.registers[x as usize] = keys
                        .iter()
                        .enumerate()
                        .filter_map(|(k, b)| if *b {Some(k)} else {None})
                        .next()
                        .unwrap() as u8
                }
            }
            InstructionTypes::SetDisplayTimer(x) => {
                self.delay_timer = self.registers[x as usize];
            }
            InstructionTypes::SetSoundTimer(x) => {
                self.delay_timer = self.registers[x as usize];
            }
            InstructionTypes::AddIndex(x) => {
                self.index = (self.index + (self.registers[x as usize] as u16)) & 0xfff
            }
            InstructionTypes::LoadIndex(x) => {
                let char_offset = self.registers[x as usize] as u16 * 5;
                self.index = 0x50 + char_offset;
            }
            InstructionTypes::StoreBDC(x) => {
                let slice = &mut self.memory[(self.index) as usize..(self.index as usize + 3)];

                let val = self.registers[x as usize];
                slice[0] = val / 100;
                slice[1] = val % 100 / 10;
                slice[2] = val % 10;
            }
            InstructionTypes::StoreRange(x) => {
                for r in 0..=x as usize{
                    self.memory[self.index as usize + r] = self.registers[r];
                }
            }
            InstructionTypes::LoadRange(x) => {
                for r in 0..=x as usize{
                    self.registers[r] = self.memory[self.index as usize + r];
                }
            }
        }
        None
    }
}