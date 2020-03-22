use pansimlib;
use std::env;
use std::error::Error;
use std::process;
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    if let None = args.next() {
        eprintln!("Something went horribly wrong! Could not find name of executable.");
        process::exit(1);
    }
    let path;
    match args.next() {
        Some(a) => path = a,
        None => {
            eprintln!("Expected at least one more argument for the config file to read from.");
            process::exit(2);
        }
    }
    pansimlib::run(&path)?;
    Ok(())
}
