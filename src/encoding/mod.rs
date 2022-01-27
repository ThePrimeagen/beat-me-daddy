pub mod base_encoding;
pub mod squeeze_me_daddy;

pub fn encode(data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let str = base_encoding::encode(data)?;
    return Ok(squeeze_me_daddy::squeeze_me_daddy(&str));
}

pub fn decode(data: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // todo: i hate these errors
    let data = squeeze_me_daddy::spread_me_daddy(&data)?;
    println!("decoded: {:?}", data);
    return Ok(base_encoding::decode(&data)?);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_encoding() -> Result<(), Box<dyn std::error::Error>> {
        let encoded = "_6Az0B08CBBDAAECCFDDFAvEEEEFFDAtEEEEFFDAr0";
        print!("{}", encoded);
        println!("decoded: {:?}", decode(&encoded.to_string())?);

        return Ok(());
    }
}

