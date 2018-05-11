use error::{IMError, IMResult};
use std::env;
use toml::Value;

pub type IMSettings = Value;

pub fn get_settings() -> IMResult<IMSettings> {
    use std::fs::File;
    use std::io::prelude::*;
    use toml::Value;
    let file = format!("{}/.iman.toml", env::var("HOME")?);
    let mut f = match File::open(&file) {
        Ok(file) => file,
        Err(e) => return Err(IMError::new(&format!("Unable to open: {}: {}", &file, e))),
    };
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    let table = buffer.parse::<Value>()?;
    Ok(table)
}
