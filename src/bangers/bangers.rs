use crate::encoding::{decode, encode};

use super::{boolizer::Boolizer, consts::BEAT_COUNT};

use std::collections::{HashMap, VecDeque};

const DRUM_NAMES: [&str; 22] = [
    "bd_pure",
    "bd_boom",
    "drum_cowbell",
    "drum_roll",
    "drum_heavy_kick",
    "drum_tom_hi_soft",
    "drum_tom_hi_hard",
    "drum_tom_mid_soft",
    "drum_tom_mid_hard",
    "drum_tom_lo_soft",
    "drum_tom_lo_hard",
    "drum_splash_soft",
    "drum_splash_hard",
    "drum_snare_soft",
    "drum_snare_hard",
    "drum_cymbal_soft",
    "drum_cymbal_hard",
    "drum_cymbal_open",
    "drum_cymbal_closed",
    "drum_cymbal_pedal",
    "drum_bass_soft",
    "drum_bass_hard",
];

pub enum WriteNode {
    Thing(String, usize, bool),
    ThingDone,
    ThingFinished,
}

pub trait BangersSerializer {
    fn direction(&self) -> Direction;
    fn write(&mut self, node: WriteNode);
}

pub enum Direction {
    Row,
    Column,
}

type DrumLine = HashMap<String, [bool; BEAT_COUNT]>;

pub struct Bangers {
    drums: DrumLine,
}

type PrimeResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn serialize(map: &DrumLine) -> PrimeResult<String> {
    let mut boolizer = Boolizer::new();

    for drum in DRUM_NAMES {
        match map.get(drum) {
            Some(line) => {
                line.iter().for_each(|b| {
                    boolizer
                        .push(*b)
                        .expect("This to never fail, like the other 2 of them...");
                });
            }
            None => {}
        }
    }

    boolizer.finish()?;

    return Ok(encode(&boolizer.data)?);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_charizer() -> Result<(), Box<dyn std::error::Error>> {
        let out = deserialize(&"88z0z0z0z0z0z0z0z0z0z0z0z0z0z0z0x0".to_string())?;
        let bd_boom = out.get(DRUM_NAMES[0]).unwrap();

        assert_eq!(bd_boom[0], true);
        assert_eq!(bd_boom[4], true);

        return Ok(());
    }
}
// The best song: ♥s89z0z0z0z0z0z0z0z0u080808080808080az0z0g02t0adadadadadadadaf08080g80808d8daz0n1
// THrobber by TJ: ♥z0z0z0s0221h020080227g08i0200fj0408h0800854z008z0z0q0808080808080808z0z0x0aeaeaeaeaeaeaeafz0n0t8

// TODO: AGain... the errors.  You should really learn how to do this...
pub fn deserialize(str: &String) -> Result<DrumLine, Box<dyn std::error::Error>> {
    let hex_str = decode(str)?;
    let bools = hex_str
        .iter()
        .flat_map(|byte| {
            let mut bools: Vec<bool> = vec![];

            for i in 0..8 {
                bools.push((byte >> (7 - i)) & 0x1 == 0x1);
            }

            return bools;
        })
        .collect::<VecDeque<bool>>();

    let mut acc: Vec<Vec<bool>> = Vec::new();
    let mut curr: Vec<bool> = Vec::new();

    for b in &bools {
        if curr.len() == BEAT_COUNT {
            acc.push(curr);
            curr = Vec::new();
        }
        curr.push(*b);
    }

    if curr.len() > 0 {
        for _ in curr.len()..BEAT_COUNT {
            curr.push(false);
        }
        acc.push(curr);
    }

    if bools.len() != BEAT_COUNT * DRUM_NAMES.len() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "WHY DID YOU TRY TO CHEAT ON ME?",
        )));
    }

    let mut drum_line: DrumLine = HashMap::new();
    for (idx, drum) in DRUM_NAMES.iter().enumerate() {
        for beat_idx in 0..BEAT_COUNT {
            drum_line
                .entry(drum.to_string())
                .or_insert([false; BEAT_COUNT])[beat_idx] =
                *bools.get(idx * BEAT_COUNT + beat_idx).unwrap();
        }
    }

    return Ok(drum_line);
}

impl Bangers {
    pub fn new() -> Bangers {
        let mut bangers = Bangers {
            drums: HashMap::new(),
        };
        bangers.reset();
        return bangers;
    }

    pub fn reset(&mut self) {
        let mut drums = HashMap::new();
        for name in &DRUM_NAMES {
            drums.insert(name.to_string(), [false; BEAT_COUNT]);
        }
        self.drums = drums;
    }

    // For twitch
    pub fn bang(&mut self, bang: &String) -> Result<(), Box<dyn std::error::Error>> {
        self.drums = deserialize(&bang.chars().skip(1).collect::<String>())?;
        return Ok(());
    }

    // For the cli
    pub fn toggle(&mut self, drum_idx: usize, column: usize) {
        let drums = self.drums.get_mut(DRUM_NAMES[drum_idx]).unwrap();
        drums[column] = !drums[column];
    }

    pub fn get_keys() -> &'static [&'static str] {
        return &DRUM_NAMES;
    }

    pub fn get_count() -> usize {
        return BEAT_COUNT;
    }

    pub fn serialize<T: BangersSerializer>(&self, writer: &mut T) {
        let d: Direction = writer.direction();

        match d {
            Direction::Column => {
                for pos in 0..BEAT_COUNT {
                    for drum_name in DRUM_NAMES {
                        //for (drum, positions) in &self.drums {
                        writer.write(WriteNode::Thing(
                            drum_name.to_string(),
                            pos,
                            self.drums.get(drum_name).unwrap()[pos],
                        ));
                        //}
                    }
                    writer.write(WriteNode::ThingDone);
                }
            }

            Direction::Row => {
                for drum_name in DRUM_NAMES {
                    for pos in 0..BEAT_COUNT {
                        writer.write(WriteNode::Thing(
                            drum_name.to_string(),
                            pos,
                            self.drums.get(drum_name).unwrap()[pos],
                        ));
                    }
                    writer.write(WriteNode::ThingDone);
                }
            }
        }

        writer.write(WriteNode::ThingFinished);
    }
}
