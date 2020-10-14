mod algorithms;
mod utils;

pub fn run_interactive() {
    let (m1, m2) = utils::read_data();
    utils::run_tests(&m1, &m2);
}


pub fn run_tests() {
    let n = utils::get_n();
    let sizes = utils::get_sizes();
    
    utils::measure(n, sizes);
}
