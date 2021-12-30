use std::{collections::HashMap, str::FromStr};

const BEAT_COUNT: usize = 128;
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

pub struct Bangers {
    drums: HashMap<String, [bool; BEAT_COUNT]>,
}

struct Bang {
    drum_type: String,
    positions: Vec<usize>,
}

impl Bang {
    fn is_valid(&self) -> bool {
        return DRUM_NAMES.iter().any(|s| *s == self.drum_type)
            && !self.positions.iter().any(|pos| pos >= &BEAT_COUNT);
    }
}

impl FromStr for Bang {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("!bang") {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Must start with !bang",
            ));
        }

        if s.chars().filter(|x| x.is_ascii_whitespace()).count() < 2 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Not a complete command.  Expected !bang <drum_name> <at_least_one_position>",
            ));
        }

        let (drum_type, positions) = s.split_once(" ").unwrap().1.split_once(" ").unwrap();
        let positions: Vec<usize> = positions
            .split(" ")
            .flat_map(str::parse)
            .map(|x| x - 1)
            .collect();

        return Ok(Bang {
            positions,
            drum_type: drum_type.to_string(),
        });
    }
}

impl Bangers {
    pub fn new() -> Bangers {
        return Bangers {
            drums: HashMap::new(),
        };
    }

    pub fn reset(&mut self) {
        self.drums = HashMap::new();
    }

    pub fn bang(&mut self, bang: &String) -> Result<(), Box<dyn std::error::Error>> {
        let bang: Bang = bang.parse()?;
        if bang.is_valid() {
            for pos in &bang.positions {
                self.drums
                    .entry(bang.drum_type.clone())
                    .or_insert([false; BEAT_COUNT])[*pos] = true;
            }
        }

        return Ok(());
    }

    pub fn serialize(&self) -> String {
        let mut banger: Vec<String> = vec![
            "live_loop :bangers do".to_string(),
            "    use_bpm 60".to_string(),
        ];

        for pos in 0..BEAT_COUNT {
            for (drum, positions) in &self.drums {
                if positions[pos] {
                    banger.push(format!("sample :{}", drum).to_string());
                }
            }
            banger.push("sleep 0.125".to_string());
        }

        banger.push("end".to_string());
        return banger.join("\n");
    }
}
