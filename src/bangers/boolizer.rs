use crate::encoding::encode;

use super::consts::STARTING_UTF;

type PrimeResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn is_bang_command(str: &String) -> bool {
    return str.starts_with(STARTING_UTF);
}

pub struct Boolizer {
    pub data: Vec<u8>,
    offset: usize,
    tmp: u8,
}

impl Boolizer {
    pub fn new() -> Boolizer {
        return Boolizer {
            data: Vec::new(),
            offset: 0,
            tmp: 0,
        };
    }

    pub fn push(&mut self, b: bool) -> Result<(), std::io::Error> {
        if b {
            let position = 8 - self.offset - 1;
            self.tmp |= 0x1 << position;
        }

        self.offset += 1;
        if self.offset == 8 {
            self.finish()?;
        }

        return Ok(());
    }

    pub fn finish(&mut self) -> Result<(), std::io::Error> {
        if self.offset != 0 {
            self.data.push(self.tmp);

            self.offset = 0;
            self.tmp = 0;
        }

        return Ok(());
    }

    pub fn encode(&self) -> PrimeResult<String> {
        return encode(&self.data);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_boolizer() -> Result<(), Box<dyn std::error::Error>> {
        let mut bools: Vec<bool> = vec![false; 23 * 4];

        // 23 a
        for i in 0..23 {
            bools[i * 4] = true;
            bools[i * 4 + 2] = true;
        }

        let mut boolizer = Boolizer::new();
        for b in bools {
            boolizer.push(b)?;
        }
        boolizer.finish()?;

        let expected = "zaa0";

        assert_eq!(expected, boolizer.encode()?);

        return Ok(());
    }
}

