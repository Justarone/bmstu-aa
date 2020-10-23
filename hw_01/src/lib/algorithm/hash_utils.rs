const BIG_PRIME: usize = 1_000_000_009;
const A_COEFF: usize = 263;

pub fn hash(string: &str) -> usize {
    let mut result = 0;
    for &c in string.as_bytes() {
        result = (result * A_COEFF + c as usize) % BIG_PRIME;
    }

    result
}

fn get_mult(len: usize) -> usize {
    let mut res = 1;
    for _ in 0..len {
        res = (res * A_COEFF) % BIG_PRIME;
    }
    res
}

pub fn precompute_hashes(string: &str, len: usize) -> Vec<usize> {
    let mut result = Vec::with_capacity(string.len() - len + 1);
    let mut res: usize = hash(&string[0..len]);
    let max_mult = get_mult(len);
    result.push(res);

    for (&cL, &c0) in string[len..].as_bytes().iter().zip(string.as_bytes().iter()) {
        res = (res * A_COEFF) % BIG_PRIME + (cL as u8) as usize - (max_mult * (c0 as u8) as usize) % BIG_PRIME;
        result.push(res);
    }

    result
}
