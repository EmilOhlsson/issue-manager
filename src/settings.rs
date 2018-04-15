fn get_settings() -> IMResult<toml::Value> {
    use std::fs::File;
    use std::io::prelude::*;
    use toml::Value;

    let mut f = File::open(format!("{}/.iman.toml", env::var("HOME")?))?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();

    println!("Read: {}", buffer);
    let table = buffer.parse::<Value>()?;
    println!("{:?}", table);
    println!("Deeenk {}", table["foo"]["yoo"]);
    table;
}

