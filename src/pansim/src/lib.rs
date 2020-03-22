use std::error::Error;
mod config;
pub use config::*;

mod population;

pub fn run(cfg: Config) -> Result<(), Box<dyn Error>> {
    let mut s = population::Society::new(cfg.clone());
    s.init();
    let mut last = s.to_string();
    loop {
        println!("{}", last);
        s.next_day();
        let new = s.to_string();
        if new == last {
            break;
        }
        last = new;
    }
    Ok(())
}
