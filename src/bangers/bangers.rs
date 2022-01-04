use std::{collections::HashMap};

use super::{boolizer::{Boolizer, Charizer}, consts::{BIT_LENGTH, BEAT_COUNT}};

const DRUM_NAMES: [&str; 22] = [
    "bd_pure",
    "bd_boom",
    "drum_cowbell",
    "drum_roll",
    "drum_heavy_kick",
    "drum_tom_mid_soft",
    "drum_tom_mid_hard",
    "drum_tom_lo_soft",
    "drum_tom_lo_hard",
    "drum_tom_hi_soft",
    "drum_tom_hi_hard",
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

pub fn serialize(map: &DrumLine) -> Result<String, std::io::Error> {
    let mut boolizer = Boolizer::new(BIT_LENGTH);

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

    return Ok(boolizer.data.iter().collect::<String>());
}

pub fn deserialize(str: &String) -> Result<DrumLine, std::io::Error> {
    let charizer: Charizer = str.parse()?;
    let drum_lines = charizer.subdivide(BEAT_COUNT);
    let mut drums: DrumLine = HashMap::new();

    for (idx, drum) in DRUM_NAMES.iter().enumerate() {
        let line = drum_lines
            .get(idx)
            .expect("The number of drum lines should never differ from the drum set")
            .iter()
            .enumerate()
            .fold([false; BEAT_COUNT], |mut beats, (idx, on)| {
                beats[idx] = *on;
                return beats;
            });

        drums.insert(drum.to_string(), line);
    }

    return Ok(drums);
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
