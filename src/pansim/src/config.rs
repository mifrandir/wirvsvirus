use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use toml;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub save_to_file: bool,
    pub population: Population,
    pub virus: Virus,
}

impl Config {
    pub fn parse(path: &str) -> Result<Self, Box<dyn Error>> {
        let contents = fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&contents)?;
        config.population.adjust_sizes();
        Ok(config)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Virus {
    pub contagiousness: f32,
    pub contagious_for: i32,
    pub sick_for: i32,
    pub lethality: [f32; 10],
    pub treatment_decay: f32,
    pub treatment_efficiency: f32,
    pub treatment_quarantine_efficiency: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Population {
    pub size: u32,
    pub age_distribution: [f32; 10],
    pub city_size: u32,
    pub district_size: u32,
    pub household_size: u32,
    pub mean_household_mobility: f32,
    pub mean_district_mobility: f32,
    pub mean_city_mobility: f32,
    pub mean_national_mobility: f32,
    pub city_medical_capacity: usize,
}

impl Population {
    fn adjust_sizes(&mut self) {
        self.district_size += self.district_size % self.household_size;
        self.city_size += self.city_size % self.district_size;
        self.size += self.size % self.city_size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_example_config() -> Result<(), Box<dyn Error>> {
        let cfg = Config::parse("Config.toml")?;
        assert_eq!(cfg.population.size, 10000);
        assert_eq!(
            cfg.virus.lethality,
            [0.002, 0.002, 0.002, 0.004, 0.013, 0.036, 0.08, 0.148, 0.148, 0.148]
        );
        Ok(())
    }
}
