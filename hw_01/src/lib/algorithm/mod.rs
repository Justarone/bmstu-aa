mod hash_utils;

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

fn compare_hashes(hashes: &[usize], pattern_hash: usize) -> Vec<usize> {
    let mut result = Vec::new();

    for (i, &hash) in hashes.iter().enumerate() {
        if hash == pattern_hash {
            result.push(i);
        }
    }

    result
}

fn compare_patterns(mut positions: Vec<usize>, data: &StrPat) -> Vec<usize> {
    let pattern_len = data.pattern.len();
    positions.retain(|&elem| data.string[elem..(elem + pattern_len)] == data.pattern[..]);
    positions
}

pub fn rabin_karp(data: &StrPat) -> Vec<usize> {
    let pattern_hash = hash_utils::hash(&data.pattern);
    let hashes = hash_utils::precompute_hashes(&data.string, data.pattern.len());
    let result = compare_hashes(&hashes, pattern_hash);
    let result = compare_patterns(result, &data);
    result
}
