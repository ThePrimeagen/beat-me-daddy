use std::str::FromStr;

use super::constants::{BIT_LENGTH, STARTING_UTF, STARTING_UTF_OFFSET};

pub struct Charizer {
    pub data: Vec<bool>,
    bit_length: usize,
}

pub fn is_bang_command(str: &str) -> bool {
    return str.starts_with(STARTING_UTF);
}

impl FromStr for Charizer {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut charizer = Charizer::new(BIT_LENGTH);
        for c in s.chars() {
            charizer.push(c)?;
        }

        return Ok(charizer);
    }
}

impl Charizer {
    pub fn new(bit_length: usize) -> Charizer {
        return Charizer {
            data: Vec::new(),
            bit_length,
        };
    }

    pub fn push(&mut self, c: char) -> Result<(), std::io::Error> {
        let c = Charizer::to_num(c)?;

        for idx in 0..self.bit_length {
            let shift_units: u32 = (self.bit_length - idx - 1) as u32;
            let bit: u32 = c >> shift_units;
            self.data.push(bit & 0x1 == 0x1);
        }

        return Ok(());
    }

    pub fn subdivide(&self, line_length: usize) -> Vec<Vec<bool>> {
        let mut out: Vec<Vec<bool>> = Vec::new();
        let mut curr: Vec<bool> = Vec::new();

        for b in &self.data {
            if curr.len() == line_length {
                out.push(curr);
                curr = Vec::new();
            }
            curr.push(*b);
        }

        if !curr.is_empty() {
            curr.resize(line_length, false);
            out.push(curr);
        }

        return out;
    }

    fn to_num(c: char) -> Result<u32, std::io::Error> {
        if (c as u32) < STARTING_UTF_OFFSET {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "required ♥ or higher char",
            ));
        }

        return Ok(c as u32 - STARTING_UTF_OFFSET);
    }
}

pub struct Boolizer {
    pub data: Vec<char>,
    offset: usize,
    tmp: u32,
    bit_length: usize,
}

impl Boolizer {
    pub fn new(bit_length: usize) -> Boolizer {
        return Boolizer {
            data: Vec::new(),
            offset: 0,
            tmp: 0,
            bit_length,
        };
    }

    fn to_char(num: u32) -> Result<Option<char>, std::io::Error> {
        if num >= 1024 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Only accepts numbers between 0..1024",
            ));
        }

        return Ok(char::from_u32(STARTING_UTF_OFFSET + num));
    }

    pub fn push(&mut self, b: bool) -> Result<(), std::io::Error> {
        if b {
            let position = self.bit_length - self.offset - 1;
            self.tmp |= 0x1 << position;
        }

        self.offset += 1;
        if self.offset == self.bit_length {
            self.finish()?;
        }

        return Ok(());
    }

    pub fn finish(&mut self) -> Result<(), std::io::Error> {
        if self.offset != 0 {
            self.data
                .push(Boolizer::to_char(self.tmp)?.expect("Boolizer::to_char should never fail."));

            self.offset = 0;
            self.tmp = 0;
        }

        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_charizer() -> Result<(), Box<dyn std::error::Error>> {
        let chars = [
            Boolizer::to_char(0x1 << 9)?.unwrap(),
            Boolizer::to_char(0x1)?.unwrap(),
            Boolizer::to_char(0x1)?.unwrap(),
            Boolizer::to_char(0x1 << 9)?.unwrap(),
        ];

        let mut charizer = Charizer::new(BIT_LENGTH);
        for c in chars {
            charizer.push(c)?;
        }

        let drum_lines = charizer.subdivide(20);

        assert!(drum_lines[0][0]);
        assert!(drum_lines[0][19]);

        assert!(drum_lines[1][9]);
        assert!(drum_lines[1][10]);

        return Ok(());
    }

    #[test]
    fn test_boolizer() -> Result<(), Box<dyn std::error::Error>> {
        let mut bools: Vec<bool> = vec![false; 40];
        bools[0] = true;
        bools[19] = true;
        bools[29] = true;
        bools[30] = true;

        let mut boolizer = Boolizer::new(BIT_LENGTH);
        for b in bools {
            boolizer.push(b)?;
        }
        boolizer.finish()?;

        let chars = [
            Boolizer::to_char(0x1 << 9)?.unwrap(),
            Boolizer::to_char(0x1)?.unwrap(),
            Boolizer::to_char(0x1)?.unwrap(),
            Boolizer::to_char(0x1 << 9)?.unwrap(),
        ];

        for (idx, c) in boolizer.data.iter().enumerate() {
            assert!(*c == chars[idx]);
        }

        return Ok(());
    }
}
