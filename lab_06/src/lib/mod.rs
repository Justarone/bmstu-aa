mod ant_solver;
mod brute_solver;
mod constants;
mod utils;

use ant_solver::AntSolver;
use brute_solver::BruteSolver;
use utils::{read_config, read_data};

pub fn run() {
    let data = read_data();
    println!("data:");
    let cnt = data.iter().map(|row| println!("{:?}", row)).count();
    println!("data size: {}\n", cnt);

    let brute = BruteSolver::new(&data);
    let bpath = brute.solve();
    println!("brute: {:?}\n", bpath);

    let config = read_config();
    let ant = AntSolver::new(&data, config);
    let apath = ant.solve();
    println!("ant algorithm: {:?}", apath);
}
