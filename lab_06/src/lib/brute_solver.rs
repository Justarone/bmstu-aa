use super::Cost;
use itertools::Itertools;

pub struct BruteSolver<'a> {
    data: &'a [Vec<Cost>],
}

impl<'a> BruteSolver<'a> {
    pub fn new(data: &'a [Vec<Cost>]) -> Self {
        Self { data }
    }

    pub fn solve(&self) -> (Cost, Vec<usize>) {
        let (mut best_l, mut best_t) = (Cost::MAX, Vec::new());
        for permutation in (1..self.data.len()).permutations(self.data.len() - 1) {
            let l = self.compute_dist(&permutation);
            if l < best_l {
                best_l = l;
                best_t = permutation;
            }
        }
        best_t.insert(0, 0);
        best_t.push(0);
        (best_l, best_t)
    }

    fn compute_dist(&self, t: &[usize]) -> Cost {
        match t.len() {
            0 => 0 as Cost,
            1 => 2 as Cost * self.data[0][t[0]],
            _ => {
                self.data[0][t[0]]
                    + t.windows(2)
                        .fold(0 as Cost, |acc, el| acc + self.data[el[0]][el[1]])
                    + self.data[t[t.len() - 1]][0]
            }
        }
    }
}
