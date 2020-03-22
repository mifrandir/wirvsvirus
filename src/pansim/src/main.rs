use pansimlib;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let cfg = pansimlib::Config::parse("Config.toml")?;
    pansimlib::run(cfg)?;
    Ok(())
}
