mod ant_solver;
mod brute_solver;
mod constants;
mod utils;

use std::time::Instant;

use ant_solver::AntSolver;
use brute_solver::BruteSolver;
use utils::{generate_data, read_config, read_data};

pub type Cost = usize;

pub fn run_time() {
    let mut config = read_config(constants::CONFIG_FILE);
    for s in constants::TIME_FROM..(constants::TIME_TO + 1) {
        config.m = s;
        print!("Size: {}, ", s);
        let data = generate_data(s);

        let time = Instant::now();
        let brute = BruteSolver::new(&data);
        let (lb, _) = brute.solve();
        print!("BruteTime: {} mcs, ", time.elapsed().as_micros());

        let time = Instant::now();
        let ant = AntSolver::new(&data, config.clone());
        let (la, _) = ant.solve();
        assert_eq!(la, lb, "{}", format!("{:?}", data));
        println!("AntTime: {} mcs;", time.elapsed().as_micros());
    }
}

pub fn run_parametrization() {
    let data = read_data(constants::DATA_FILE);
    let brute = BruteSolver::new(&data);
    let (best_l, _) = brute.solve();
    println!("Best Distance: {}\n", best_l);

    parametrization_test(best_l);
}

fn parametrization_test(dist: usize) {
    let from_conf = read_config(constants::CONFIG_FROM);
    let to_conf = read_config(constants::CONFIG_TO);
}
