use std::error::Error;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use toml;
mod config;
pub use config::*;

mod population;

const OUTPUT_PREFIX: &str = "output/";
const MAX_REPETITION: i32 = 14;

pub fn run(path: &str) -> Result<(), Box<dyn Error>> {
    let t = SystemTime::now();
    let cfg = config::Config::parse(path)?;
    let mut s = population::Society::new(cfg.clone());
    eprintln!("Initialized in {:.2}s", t.elapsed().unwrap().as_secs_f32());
    s.init();
    let mut history = Vec::new();
    history.push(s.csv_header());
    history.push(s.to_string());
    //println!("{}", s.to_string());
    while s.active() > 0 {
        s.next_day();
        let new = s.to_string();
        //println!("{}", new);
        history.push(new);
    }
    for _ in 1..MAX_REPETITION {
        history.pop();
    }
    let mut out = String::new();
    for l in history {
        out.push_str(&l);
        out.push('\n')
    }
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let out_path = format!("{}{}.csv", OUTPUT_PREFIX, t);
    fs::create_dir_all(OUTPUT_PREFIX).unwrap();
    fs::write(out_path, out)?;
    let config_toml = toml::to_string_pretty(&cfg)?;
    let out_path = format!("{}{}_config.toml", OUTPUT_PREFIX, t);
    fs::write(out_path, config_toml)?;
    println!("{}", t);
    Ok(())
}
