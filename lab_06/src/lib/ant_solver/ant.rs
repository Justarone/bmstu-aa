use rand::prelude::*;

#[derive(Clone)]
pub struct Ant {
    route: Vec<usize>,
    len: f64,
    left: Vec<usize>,
}

impl Ant {
    pub fn new(cities_amount: usize, start: usize) -> Self {
        Self {
            route: vec![start],
            len: 0_f64,
            left: (0..cities_amount).filter(|&e| e != start).collect(),
        }
    }

    pub fn walk(
        &mut self,
        nd: &[Vec<f64>],
        pd: &[Vec<f64>],
        alpha: f64,
        beta: f64,
        rng: &mut ThreadRng,
    ) {
        for _ in 0..self.left.len() {
            self.next(nd, pd, alpha, beta, rng);
        }
        let last_visited = self.route[self.route.len() - 1];
        self.len += 1.0 / nd[last_visited][self.route[0]];
    }

    pub fn data(&self) -> (f64, &[usize]) {
        (self.len, &self.route)
    }

    fn next(
        &mut self,
        nd: &[Vec<f64>],
        pd: &[Vec<f64>],
        alpha: f64,
        beta: f64,
        rng: &mut ThreadRng,
    ) {
        let cur = self.route[self.route.len() - 1];
        let denominator = self.left.iter().fold(0.0, |acc, &e| {
            acc + f64::powf(pd[cur][e], alpha) * f64::powf(nd[cur][e], beta)
        });
        let mut pick: f64 = rng.gen();
        for (index, &j) in self.left.iter().enumerate() {
            let cur_prob = f64::powf(pd[cur][j], alpha) * f64::powf(nd[cur][j], beta);
            pick -= cur_prob / denominator;
            if pick < 0_f64 {
                self.pick(index, 1.0 / nd[cur][j]);
                return;
            }
        }
        let index = self.left.len() - 1;
        self.pick(index, 1.0 / nd[cur][self.left[index]]);
    }

    fn pick(&mut self, index: usize, diff: f64) {
        self.route.push(self.left.remove(index));
        self.len += diff;
    }
}
