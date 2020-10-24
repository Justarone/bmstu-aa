use super::additional_structs::{ StrPat, RabinKarpTaskResult };

pub trait Task3<T: Default> {
    fn part1(&mut self);
    fn part2(&mut self);
    fn part3(&mut self);

    fn result(&self) -> T {
        T::default()
    }
}

#[derive(Debug)]
pub struct RabinKarpTask {
    data: StrPat,
    hashes: Option<Vec<usize>>,
    result: Option<Vec<usize>>,
}

impl RabinKarpTask {
    const BIG_PRIME: usize = 1_000_000_009;
    const A_COEFF: usize = 263;

    pub fn new(string: String, pattern: String) -> Self {
        Self {
            data: StrPat::new(string, pattern),
            hashes: None,
            result: None,
        }
    }

    fn hash(string: &str) -> usize {
        let mut result = 0;
        for &c in string.as_bytes() {
            result = (result * Self::A_COEFF + c as usize) % Self::BIG_PRIME;
        }
        result
    }

    fn get_mult(len: usize) -> usize {
        let mut res = 1;
        for _ in 0..len {
            res = (res * Self::A_COEFF) % Self::BIG_PRIME;
        }
        res
    }

    pub fn precompute_hashes(&mut self) {
        let pattern_len = self.data.pattern.len();
        let mut result = Vec::with_capacity(self.data.string.len() - pattern_len + 1);
        let mut res: usize = Self::hash(&self.data.string[0..pattern_len]);
        let max_mult = Self::get_mult(pattern_len);
        result.push(res);

        for (&c_l, &c_0) in self.data.string[pattern_len..]
            .as_bytes().iter().zip(self.data.string.as_bytes().iter()) {
            res = ((res * Self::A_COEFF) % Self::BIG_PRIME + (c_l as u8) as usize).wrapping_sub((max_mult * (c_0 as u8) as usize) % Self::BIG_PRIME) % Self::BIG_PRIME;
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
            result.retain(|&elem| self.data.string[elem..(elem + pattern_len)] == self.data.pattern[..]);
            self.result = Some(result);
        }
    }

    #[allow(dead_code)]
    fn run_all(&mut self) {
        self.precompute_hashes();
        self.compare_hashes();
        self.compare_patterns();
    }
}


impl Task3<RabinKarpTaskResult> for RabinKarpTask {
    fn part1(&mut self) {
        self.precompute_hashes();
    }

    fn part2(&mut self) {
        self.compare_hashes();
    }

    fn part3(&mut self) {
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
        }
    }
}

