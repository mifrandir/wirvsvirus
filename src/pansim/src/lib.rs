use std::error::Error;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
mod config;
pub use config::*;

mod population;

const OUTPUT_PREFIX: &str = "pansim_out/";
const MAX_REPETITION: i32 = 14;

pub fn run(path: &str) -> Result<(), Box<dyn Error>> {
    let cfg = config::Config::parse(path)?;
    let mut s = population::Society::new(cfg.clone());
    s.init();
    let mut history = Vec::new();
    history.push(s.csv_header());
    history.push(s.to_string());
    println!("{}", s.to_string());
    let mut re = 0;
    loop {
        s.next_day();
        let new = s.to_string();
        if new == *history.last().unwrap() {
            re += 1;
            if re == MAX_REPETITION {
                break;
            }
        } else {
            re = 0;
        }
        println!("{}", new);
        history.push(new);
    }
    for _ in 1..MAX_REPETITION {
        history.pop();
    }
    if cfg.save_to_file {
        let mut out = String::new();
        for l in history {
            out.push_str(&l);
            out.push('\n')
        }
        let out_path = format!(
            "{}{}",
            OUTPUT_PREFIX,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        fs::create_dir_all(OUTPUT_PREFIX).unwrap();
        fs::write(out_path, out)?;
    }
    Ok(())
}
