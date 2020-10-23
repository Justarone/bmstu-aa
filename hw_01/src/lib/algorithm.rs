const BIG_PRIME: usize = 1_000_000_009;
const A_COEFF: usize = 263;

pub struct StrPat {
    pub string: String,
    pub pattern: String,
}

impl StrPat {
    pub fn new(string: String, pattern: String) -> Self {
        Self {
            string,
            pattern,
        }
    }
}


pub fn rabin_karp(data: &StrPat) -> Vec<usize> {
    let mut pattern_hash = 0;
    for &c in data.pattern.as_bytes() {
        pattern_hash = (pattern_hash * A_COEFF + c as usize) % BIG_PRIME;
    }
    let pattern_len = data.pattern.len();
    let mut hashes = Vec::with_capacity(data.string.len() - pattern_len + 1);
    let mut max_mult = 1;
    for _ in 0..pattern_len {
        max_mult = (max_mult * A_COEFF) % BIG_PRIME;
    }
    let mut current_hash = 0;
    for &c in data.string[0..pattern_len].as_bytes() {
        current_hash = (current_hash * A_COEFF + c as usize) % BIG_PRIME;
    }
    hashes.push(current_hash);
    for (&c_l, &c_0) in data.string[pattern_len..].as_bytes().iter().zip(data.string.as_bytes().iter()) {
        current_hash = (current_hash * A_COEFF) % BIG_PRIME + (c_l as u8) as usize - (max_mult * (c_0 as u8) as usize) % BIG_PRIME;
        hashes.push(current_hash);
    }
    let mut result = Vec::new();
    for (i, &hash) in hashes.iter().enumerate() {
        if hash == pattern_hash {
            if data.string[i..(i + pattern_len)] == data.pattern[..] {
                result.push(i);
            }
        }
    }
    result
}
