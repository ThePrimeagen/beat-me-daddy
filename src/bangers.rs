
pub struct Bangers {
    drum_heavy_kick: [bool; 256],
}

impl Bangers {
    pub fn new() -> Bangers {
        return Bangers {
            drum_heavy_kick: [false; 256]
        }
    }

    pub fn bang(&mut self, bang: &String) -> Result<(), Box<dyn std::error::Error>> {
        println!("bang {}", bang);
        if bang.starts_with("!bang") {
            println!("bang started with !bang");
            let (_, position) = bang.split_once(" ").unwrap();
            let position: usize = position.parse()?;
            println!("bang position {}", position);

            if position < 256 {
                self.drum_heavy_kick[position] = true;
            }
        }

        return Ok(());
    }

    pub fn serialize(&self) -> String {
        let mut banger: Vec<String> = vec!["live_loop :bangers do".to_string()];

        for play in self.drum_heavy_kick {
            banger.push(if play {
                "sample :drum_heavy_kick, rate 1".to_string()
            } else {
                "sleep 0.125".to_string()
            });
        }

        return banger.join("\n");
    }
}


