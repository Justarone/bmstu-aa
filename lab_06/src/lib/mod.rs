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
        print!("Size: {}, ", s);
        let data = generate_data(s, constants::VALS_FROM, constants::VALS_TO);
        config.m = data.len();

        let time = Instant::now();
        let brute = BruteSolver::new(&data);
        let (_lb, _tb) = brute.solve();
        print!("BruteTime: {} mcs, ", time.elapsed().as_nanos());

        let time = Instant::now();
        let ant = AntSolver::new(&data, config.clone());
        let (_la, _ta) = ant.solve();
        assert_eq!(_la, _lb, "{:?}\n{:?}\n{:?}", data, _ta, _tb);
        println!("AntTime: {} mcs;", time.elapsed().as_nanos());
    }
}

pub fn run_parametrization() {
    let mut conf = read_config(constants::CONFIG_FILE);
    conf.m = constants::TEST_SIZE;
    for (from, to) in [
        (constants::VALS_FROM, constants::VALS_TO),
        (constants::BIG_VALS_FROM, constants::BIG_VALS_TO),
    ]
    .iter()
    {
        let data = generate_data(constants::TEST_SIZE, *from, *to);
        print_data(&data);
        let brute = BruteSolver::new(&data);
        let (best_l, _) = brute.solve();
        println!("Best Distance: {}\n", best_l);
        conf.alpha = constants::ALPHA_START;
        while conf.alpha < constants::ALPHA_END + constants::ALPHA_STEP / 2_f64 {
            conf.beta =
                f64::round((1.0 - conf.alpha) * constants::PRESISION) / constants::PRESISION;
            conf.p = constants::P_START;
            while conf.p < constants::P_END + constants::P_STEP / 2_f64 {
                let ant_solver = AntSolver::new(&data, conf.clone());
                let (len, _) = ant_solver.solve();
                println!(
                    "{:<4} & {:<4} & {:<4} & {:<5} & {:<5} \\\\",
                    conf.alpha,
                    conf.beta,
                    conf.p,
                    len,
                    len - best_l
                );
                conf.p = f64::round((conf.p + constants::P_STEP) * constants::PRESISION)
                    / constants::PRESISION;
            }
            conf.alpha = f64::round((conf.alpha + constants::ALPHA_STEP) * constants::PRESISION)
                / constants::PRESISION;
        }
    }
}

fn print_data(data: &[Vec<Cost>]) {
    data.iter().for_each(|row| {
        print!("{:>5} ", row[0]);
        row.iter().skip(1).for_each(|e| print!("& {:>5} ", e));
        println!("\\\\");
    })
}

pub fn run_interactive() {
    let data = read_data(constants::DATA_FILE);
    println!("Data:");
    data.iter().for_each(|row| {
        row.iter().for_each(|elem| print!("{:>5} ", elem));
        println!("");
    });
    println!("\n");

    let brute = BruteSolver::new(&data);
    let (len, path) = brute.solve();
    println!("BRUTE:\nLength: {:>5}, Path: {:?}", len, path);

    let config = read_config(constants::CONFIG_FILE);
    let ant_solver = AntSolver::new(&data, config);
    let (len, path) = ant_solver.solve();
    println!("ANTS:\nLength: {:>5}, Path: {:?}", len, path);
}
