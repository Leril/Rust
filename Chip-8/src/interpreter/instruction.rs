
type Register = u8;
type Addr = u16;

#[derive(Debug, PartialEq)]
pub enum InstructionTypes{
    SysOld,//0nnn
    Clear,//00E0
    Return,//00EE
    Jump(Addr),//1nnn
    Call(Addr),//2nnn
    SkipIfKK(Register, u8),//3xkk
    SkipIfNotKK(Register, u8),//4xkk
    SkipIfCompare(Register, Register),//5xy0
    SetKK(Register, u8),//6xkk
    AddKK(Register, u8),//7xkk
    SetTo(Register, Register),//8xy0
    BitOr(Register, Register),//8xy1
    BitAnd(Register, Register),//8xy2
    BitXor(Register, Register),//8xy3
    BitAdd(Register, Register),//8xy4
    BitSub(Register, Register),//8xy5
    BitShr(Register, Register),//8xy6
    BitSubN(Register, Register),//8xy7
    BitSHL(Register, Register),//8xyL
    SkipNextIfNotEqual(Register, Register),//9xyE
    SetN(u16),//Annn
    JumpN(u16),//Bnnn
    RandomKK(Register, u8),//Cxkk
    Draw(Register, Register, u8),//Cxkk
    SkipIfPressed(Register),//Ex9E
    SkipIfNotPressed(Register),//ExA1
    GetTimer(Register),//Fx07
    WaitFor(Register),//Fx0A
    SetDisplayTimer(Register),//Fx15
    SetSoundTimer(Register),//Fx18
    AddIndex(Register),//Fx1E
    LoadIndex(Register),//Fx29
    StoreBDC(Register),//Fx33
    StoreRange(Register),//Fx55
    LoadRange(Register),//Fx65
}

pub fn decode(opcode: u16) -> InstructionTypes{
    let nnn = get_twelve_bits(opcode);
    let kk = get_eight_bits(opcode);

    match get_nibbles(opcode) {
        (0, 0, 0xE, 0) => InstructionTypes::Clear,
        (0, 0, 0xE, 0xE) => InstructionTypes::Return,
        (0, _, _, _) => InstructionTypes::SysOld,
        (1, _, _, _) => InstructionTypes::Jump(nnn),
        (2, _, _, _) => InstructionTypes::Call(nnn),
        (3, x, _, _) => InstructionTypes::SkipIfKK(x, kk),
        (4, x, _, _) => InstructionTypes::SkipIfNotKK(x, kk),
        (5, x, y, 0) => InstructionTypes::SkipIfCompare(x, y),
        (6, x, _, _) => InstructionTypes::SetKK(x, kk),
        (7, x, _, _) => InstructionTypes::AddKK(x, kk),
        (8, x, y, 0) => InstructionTypes::SetTo(x, y),
        (8, x, y, 1) => InstructionTypes::BitOr(x, y),
        (8, x, y, 2) => InstructionTypes::BitAnd(x, y),
        (8, x, y, 3) => InstructionTypes::BitXor(x, y),
        (8, x, y, 4) => InstructionTypes::BitAdd(x, y),
        (8, x, y, 5) => InstructionTypes::BitSub(x, y),
        (8, x, y, 6) => InstructionTypes::BitShr(x, y),
        (8, x, y, 7) => InstructionTypes::BitSubN(x, y),
        (8, x, y, 0xE) => InstructionTypes::BitSHL(x, y),
        (9, x, y, 0) => InstructionTypes::SkipNextIfNotEqual(x, y),
        (0xA, _, _, _) => InstructionTypes::SetN(nnn),
        (0xB, _, _, _) => InstructionTypes::JumpN(nnn),
        (0xC, x, _, _) => InstructionTypes::RandomKK(x, kk),
        (0xD, x, y, n) => InstructionTypes::Draw(x, y, n),
        (0xE, x, 9, 0xE) => InstructionTypes::SkipIfPressed(x),
        (0xE, x, 0xA, 1) => InstructionTypes::SkipIfNotPressed(x),
        (0xF, x, 0, 7) => InstructionTypes::GetTimer(x),
        (0xF, x, 0, 0xA) => InstructionTypes::WaitFor(x),
        (0xF, x, 1, 5) => InstructionTypes::SetDisplayTimer(x),
        (0xF, x, 1, 8) => InstructionTypes::SetSoundTimer(x),
        (0xF, x, 1, 0xE) => InstructionTypes::AddIndex(x),
        (0xF, x, 2, 9) => InstructionTypes::LoadIndex(x),
        (0xF, x, 3, 3) => InstructionTypes::StoreBDC(x),
        (0xF, x, 5, 5) => InstructionTypes::StoreRange(x),
        (0xF, x, 6, 5) => InstructionTypes::LoadRange(x),
        (_, _, _, _) => {panic!("Invalid instruction encountered {}", opcode)}
    }
}

pub fn get_twelve_bits(n: u16) -> u16{
    n & 0xfff
}

pub fn get_eight_bits(n: u16) -> u8{
    (n & 0xff) as u8
}

pub fn get_nibbles(n: u16) -> (u8, u8, u8, u8){
    let n3 = (n >> 12) as u8;
    let n2 = ((n >> 8) & 0b1111) as u8;
    let n1 = ((n >> 4) & 0b1111) as u8;
    let n0 = (n & 0b1111) as u8;
    (n3, n2, n1, n0)
}