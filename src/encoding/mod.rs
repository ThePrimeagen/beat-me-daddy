pub mod base_encoding;
pub mod squeeze_me_daddy;

pub fn encode(data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let str = base_encoding::encode(data)?;
    return Ok(squeeze_me_daddy::squeeze_me_daddy(&str));
}

pub fn decode(data: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // todo: i hate these errors
    let data = squeeze_me_daddy::spread_me_daddy(&data)?;
    return Ok(base_encoding::decode(&data)?);
}
