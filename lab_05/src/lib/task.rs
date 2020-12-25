use super::additional_structs::{RabinKarpTaskResult, StrPat};
use chrono::{DateTime, Utc};

pub const NUMBER_OF_MEASURMENTS: usize = 6;
pub const T1_START: usize = 0;
pub const T1_END: usize = 1;
pub const T2_START: usize = 2;
pub const T2_END: usize = 3;
pub const T3_START: usize = 4;
pub const T3_END: usize = 5;

pub trait Task3<T> {
    fn part1(&mut self);
    fn part2(&mut self);
    fn part3(&mut self);

    fn run1(&mut self);
    fn run2(&mut self);
    fn run3(&mut self);

    fn result(&self) -> T;
}

#[derive(Debug, Clone)]
pub struct RabinKarpTask {
    data: StrPat,
    hashes: Option<Vec<u128>>,
    result: Option<Vec<usize>>,
    times: [DateTime<Utc>; NUMBER_OF_MEASURMENTS],
}

impl RabinKarpTask {
    const BIG_PRIME: u128 = 1_000_000_000_061;
    const A_COEFF: u128 = 10_000_004_857;

    pub fn new(string: Vec<char>, pattern: Vec<char>) -> Self {
        let current_time = Utc::now();
        Self {
            data: StrPat::new(string, pattern),
            hashes: None,
            result: None,
            times: [current_time; NUMBER_OF_MEASURMENTS],
        }
    }

    fn hash(string: &[char]) -> u128 {
        let mut result = 0;
        for &c in string.iter() {
            result = (result * Self::A_COEFF + c as u128) % Self::BIG_PRIME;
        }
        result
    }

    fn get_mult(len: usize) -> u128 {
        let mut res = 1;
        for _ in 0..len {
            res = (res * Self::A_COEFF) % Self::BIG_PRIME;
        }
        res
    }

    pub fn precompute_hashes(&mut self) {
        let pattern_len = self.data.pattern.len();
        let mut result = Vec::with_capacity(self.data.string.len() - pattern_len + 1);
        let mut res = Self::hash(&self.data.string[0..pattern_len]);
        let max_mult = Self::get_mult(pattern_len);
        result.push(res);

        for (&c_l, &c_0) in self.data.string[pattern_len..]
            .iter()
            .zip(self.data.string.iter())
        {
            res = ((res * Self::A_COEFF) % Self::BIG_PRIME + c_l as u128 + Self::BIG_PRIME
                - (max_mult * c_0 as u128) % Self::BIG_PRIME)
                % Self::BIG_PRIME;
            result.push(res);
        }

        self.hashes = Some(result);
    }

    fn compare_hashes(&mut self) {
        let pattern_hash = Self::hash(&self.data.pattern);
        if let Some(hashes) = self.hashes.as_mut() {
            let mut result = Vec::new();
            for (i, &hash) in hashes.iter().enumerate() {
                if hash == pattern_hash {
                    result.push(i);
                }
            }
            self.result = Some(result);
        }
    }

    fn compare_patterns(&mut self) {
        let tmp = self.result.take();
        if let Some(mut result) = tmp {
            let pattern_len = self.data.pattern.len();
            result.retain(|&elem| {
                self.data.string[elem..(elem + pattern_len)] == self.data.pattern[..]
            });
            self.result = Some(result);
        }
    }

    #[allow(dead_code)]
    pub fn run_all(&mut self) {
        self.precompute_hashes();
        self.compare_hashes();
        self.compare_patterns();
    }
}

impl Task3<RabinKarpTaskResult> for RabinKarpTask {
    fn part1(&mut self) {
        self.times[T1_START] = Utc::now();
        self.run1();
        self.times[T1_END] = Utc::now();
    }

    fn part2(&mut self) {
        self.times[T2_START] = Utc::now();
        self.run2();
        self.times[T2_END] = Utc::now();
    }

    fn part3(&mut self) {
        self.times[T3_START] = Utc::now();
        self.run3();
        self.times[T3_END] = Utc::now();
    }

    fn run1(&mut self) {
        self.precompute_hashes();
    }

    fn run2(&mut self) {
        self.compare_hashes();
    }

    fn run3(&mut self) {
        self.compare_patterns();
    }

    fn result(&self) -> RabinKarpTaskResult {
        let result = match self.result.as_ref() {
            None => Vec::new(),
            Some(res) => res.clone(),
        };

        RabinKarpTaskResult {
            data: self.data.clone(),
            result,
            times: self.times.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut task = RabinKarpTask::new(
            String::from("Dima Yacuba").chars().collect(),
            String::from("Pavel").chars().collect(),
        );
        task.run_all();
        let result = task.result().result;
        assert_eq!(&result, &[]);
    }

    #[test]
    fn all() {
        let mut task = RabinKarpTask::new(
            String::from("aaaaaa").chars().collect(),
            String::from("aaaaaa").chars().collect(),
        );
        task.run_all();
        let result = task.result().result;
        assert_eq!(&result, &[0]);
    }

    #[test]
    fn partial_all() {
        let mut task = RabinKarpTask::new(
            String::from("aaaaaaaaaaaa").chars().collect(),
            String::from("aaaaaa").chars().collect(),
        );
        task.run_all();
        let result = task.result().result;
        assert_eq!(&result, &[0, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn comb() {
        let mut task = RabinKarpTask::new(
            String::from("aaaabaaa").chars().collect(),
            String::from("aa").chars().collect(),
        );
        task.run_all();
        let result = task.result().result;
        assert_eq!(&result, &[0, 1, 2, 5, 6]);
    }

    #[test]
    fn some_one_timers() {
        let mut task = RabinKarpTask::new(
            String::from("someone do something sometimes")
                .chars()
                .collect(),
            String::from("some").chars().collect(),
        );
        task.run_all();
        let result = task.result().result;
        assert_eq!(&result, &[0, 11, 21]);
    }
}
