mod algorithms;
mod utils;

pub fn run_interactive() {
    let mut array = utils::read_data();
    utils::run_tests(&mut array);
}


pub fn run_tests() {
    let n = utils::get_n();
    let sizes = utils::get_size();
    
    utils::measure(n, sizes);
}
