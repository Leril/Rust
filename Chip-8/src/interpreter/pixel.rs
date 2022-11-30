use chip8_base::Pixel;

pub(crate) struct PixelIterator {
    byte: u8,
    index: u8,
}

impl PixelIterator{
    pub fn new(byte: &u8) -> Self{
        Self{
            byte: *byte,
            index: 0,
        }
    }
}

impl Iterator for PixelIterator{
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < 8{
            let bit = self.byte >> (7 - self.index) & 1;
            self.index += 1;

            assert!(bit == 1 || bit == 0);
            Some(bit.try_into().unwrap())
        }else {
            None
        }
    }
}