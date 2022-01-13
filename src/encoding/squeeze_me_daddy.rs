use std::{collections::HashMap, num::ParseIntError};

use itertools::Itertools;

#[derive(Debug)]
pub enum DaddyIssues {
    ParseInt(ParseIntError),
    MalformedString(String),
}

impl std::fmt::Display for DaddyIssues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            DaddyIssues::ParseInt(e) => {
                write!(f, "{}", e)
            },
            DaddyIssues::MalformedString(e) => {
                write!(f, "{}", e)
            },
        }
    }
}

impl std::error::Error for DaddyIssues {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        return match self {
            DaddyIssues::ParseInt(e) => Some(e),
            DaddyIssues::MalformedString(e) => {
                None
            },
        };
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl From<ParseIntError> for DaddyIssues {
    fn from(e: ParseIntError) -> Self {
        return DaddyIssues::ParseInt(e);
    }
}

fn get_pairs(s: &str, length: usize, offset: usize) -> HashMap<String, u32> {
    return s[offset..]
        .as_bytes()
        .chunks(length)
        .fold(HashMap::new(), |mut map, chunk| {
            let str = chunk
                .iter()
                .map(|x| char::from_u32(*x as u32).unwrap())
                .collect::<String>();
            if str.len() == length {
                *map.entry(str).or_insert(0) += 1;
            }
            return map;
        });
}

fn encode(s: &str, length: usize) -> HashMap<String, u32> {
    let map = get_pairs(s, length, 0);
    let map2 = get_pairs(s, length, 1);

    return map2.iter().fold(map, |mut map, (k1, v1)| {
        if let Some(v) = map.get(k1) {
            if v < v1 {
                *map.entry(k1.clone()).or_insert(*v1) = *v1;
            }
        } else {
            map.insert(k1.clone(), *v1);
        }

        return map;
    });
}

fn squeeze(s: &str, r: &str, w: &str) -> String {
    return s.split(r).join(w);
}

fn squeeze_once(s: &str, len: usize, r: &str) -> Option<(String, String)> {
    let groups = encode(s, len);
    let first = groups.iter().sorted_by(|a, b| b.1.cmp(a.1)).nth(0)?;

    if first.1 < &4 {
        return None;
    }

    return Some((first.0.to_string(), squeeze(s, first.0, r)));
}

pub fn squeeze_me_daddy(str: &String) -> String {
    let mut str = str.to_string();
    let mut count: u32 = 0;
    let mut replacements: Vec<String> = vec![];
    let start = 'A';

    loop {
        let c = char::from_u32(start as u32 + count).unwrap().to_string();
        if let Some(s) = squeeze_once(&str, 2, &c) {
            count += 1;
            replacements.push(format!("{}{}", c, s.0));
            str = s.1;
        } else {
            break;
        }
    }

    if replacements.len() > 0 {
        replacements.insert(0, replacements.len().to_string());
        return format!("_{}{}", replacements.iter().join(""), str);
    }
    return str.to_string();
}

fn split_me_daddy(str: String) -> Result<(usize, String, String), DaddyIssues> {
    let count_str: String = str
        .chars()
        .skip(1)
        .take_while(|c| c.is_digit(10))
        .collect::<String>();

    let count: usize = count_str.parse()?;

    return Ok((
        count,
        str.chars().skip(1 + count_str.len()).take(count * 3).collect::<String>(),
        str.chars().skip(1 + count_str.len() + count * 3).collect::<String>(),
    ));
}
pub fn spread_me_daddy(str: &String) -> Result<String, DaddyIssues> {
    if !str.starts_with('_') {
        return Ok(str.to_string());
    }

    let (count, replacements, mut str) = split_me_daddy(str.to_string())?;

    if replacements.len() % 3 != 0 {
        return Err(DaddyIssues::MalformedString(
            "Expected replacements string to be divisible by 3".to_string(),
        ));
    }

    for i in (0..count).rev() {
        let set = &replacements[i * 3..i * 3 + 3];
        let replacer = &set[0..1];
        let replacee = &set[1..];
        str = str.replace(replacer, replacee);
    }

    return Ok(str);
}

#[cfg(test)]
mod test_squeeze {
    use super::*;
    #[test]
    fn test_simple_no() {
        let string: String = "090909".to_string();
        let string2: String = squeeze_me_daddy(&string);

        assert_eq!(string, string2);
    }

    #[test]
    fn test_simple_yes() {
        let string: String = "09090909".to_string();
        let string2: String = squeeze_me_daddy(&string);

        assert_eq!("_1A09AAAA", string2);
    }

    #[test]
    fn test_simple_yes_longer() {
        let string: String = "0909090909099009".to_string();
        let string2: String = squeeze_me_daddy(&string);

        assert_eq!("_1A09AAAAAA90A", string2);
    }

    #[test]
    fn complex_yes() {
        let string: String = "09090909090909099009".to_string();
        let string2: String = squeeze_me_daddy(&string);

        assert_eq!("_2A09BAABBBB90A", string2);
    }

    #[test]
    fn complex_yes_longer() {
        let string: String = "0909090909090909900909090909090909".to_string();
        let string2: String = squeeze_me_daddy(&string);

        assert_eq!("_3A09BAACBBCC90CC", string2);
    }


}

#[cfg(test)]
mod test_spread {
    use super::*;
    #[test]
    fn test_spread_noop() -> Result<(), DaddyIssues> {
        let string: String = "090909".to_string();
        let string2: String = squeeze_me_daddy(&string);
        let string3: String = spread_me_daddy(&string2)?;

        assert_eq!(string3, string);

        return Ok(());
    }

    #[test]
    fn test_spread() -> Result<(), DaddyIssues> {
        let string: String = "09090909".to_string();
        let string2: String = squeeze_me_daddy(&string);
        let string3: String = spread_me_daddy(&string2)?;

        assert_eq!(string3, string);

        return Ok(());
    }

    #[test]
    fn test_spread_complex() -> Result<(), DaddyIssues> {
        let string: String = "09090909090909099009".to_string();
        let string2: String = squeeze_me_daddy(&string);
        let string3: String = spread_me_daddy(&string2)?;

        assert_eq!(string3, string);
        return Ok(());
    }

    #[test]
    fn test_spread_complex_longer() -> Result<(), DaddyIssues> {
        let string: String = "0909090909090909900909090909090909".to_string();
        let string2: String = squeeze_me_daddy(&string);
        let string3: String = spread_me_daddy(&string2)?;

        assert_eq!(string3, string);
        return Ok(());
    }

    #[test]
    fn test_real_failure_1() -> Result<(), DaddyIssues> {
        let string: String = "00808080800802a00g808008g080bbz0z0z0z0l0808082a0808082808080ad82808082aaq0104q0g4z0z0002r022az0z0005r0g1z0z0z0z0k010828085808280d080808582a08082d53z0m09z0z0z0p02s012dadadadadadadadadadadadadadadadfg05g05g05g05g05g05g05002dz0z0x".to_string();
        let string2: String = squeeze_me_daddy(&string);
        let string3: String = spread_me_daddy(&string2)?;

        assert_eq!(string3, string);
        return Ok(());
    }
}
