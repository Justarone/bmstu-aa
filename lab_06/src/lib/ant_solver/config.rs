use serde_derive::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub alpha: f64,
    pub beta: f64,
    pub e: usize, // number of elite ants
    pub p: f64,   // evaporation rate \in [0..1]
    pub m: usize, // number of ants
    pub tmax: usize,
    pub pheromon_start: f64,
    pub pheromon_min: f64,
}
