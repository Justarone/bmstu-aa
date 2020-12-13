use itertools::Itertools;
use rand::prelude::*;
use serde_derive::Deserialize;

pub struct BruteSolver<'a> {
    data: &'a [Vec<f64>],
}

impl<'a> BruteSolver<'a> {
    pub fn new(data: &'a [Vec<f64>]) -> Self {
        Self { data }
    }

    pub fn solve(&self) -> (f64, Vec<usize>) {
        let (mut best_l, mut best_t) = (f64::MAX, Vec::new());
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

    fn compute_dist(&self, t: &[usize]) -> f64 {
        match t.len() {
            0 => 0.0,
            1 => 2_f64 * self.data[0][t[0]],
            _ => {
                self.data[0][t[0]]
                    + t.windows(2)
                        .fold(0_f64, |acc, el| acc + self.data[el[0]][el[1]])
                    + self.data[t[t.len() - 1]][0]
            }
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    alpha: f64,
    beta: f64,
    e: usize, // number of elite ants
    p: f64,   // evaporation rate \in [0..1]
    m: usize, // number of ants
    tmax: usize,
    pheromon_start: f64,
    pheromon_min: f64,
}

pub struct AntSolver {
    ndata: Vec<Vec<f64>>,
    q: f64,
    config: Config,
}

#[derive(Clone)]
struct Ant {
    route: Vec<usize>,
    len: f64,
    left: Vec<usize>,
}

impl Ant {
    fn new(cities_amount: usize, start: usize) -> Self {
        Self {
            route: vec![start],
            len: 0_f64,
            left: (0..cities_amount).filter(|&e| e != start).collect(),
        }
    }

    fn walk(
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

    #[allow(dead_code)]
    fn len(&self) -> f64 {
        self.len
    }

    #[allow(dead_code)]
    fn reset(&mut self) {
        self.len = 0.0;
        self.left = self.route.clone();
        let start = self.route[0];
        self.route = vec![start];
    }

    fn data(&self) -> (f64, &[usize]) {
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

impl AntSolver {
    pub fn new(data: &[Vec<f64>], config: Config) -> Self {
        let ndata = data
            .iter()
            .map(|row| row.iter().map(|&e| 1_f64 / e).collect())
            .collect();
        let q = data
            .iter()
            .fold(0.0, |acc, row| acc + row.iter().sum::<f64>() as f64);
        let q = q - (0..data.len()).fold(0.0, |acc, index| acc + data[index][index]);
        let q = q / data.len() as f64;
        Self { ndata, q, config }
    }

    pub fn solve(&self) -> (f64, Vec<usize>) {
        let (mut rng, mut pheromon_data, mut best_t, mut best_l) = self.init_params();
        for _ in 0..self.config.tmax {
            let mut ants = self.generate_ants(&mut rng);
            for a in ants.iter_mut() {
                // Run ants
                a.walk(
                    &self.ndata,
                    &pheromon_data,
                    self.config.alpha,
                    self.config.beta,
                    &mut rng,
                );
            }

            // Find best T* and L* after day
            let best_data = ants
                .iter()
                .max_by(|a, b| match a.data().0 < b.data().0 {
                    true => std::cmp::Ordering::Less,
                    _ => std::cmp::Ordering::Greater,
                })
                .unwrap()
                .data();
            if best_data.0 < best_l {
                best_l = best_data.0;
                best_t = best_data.1.to_vec();
            }

            self.evaporation(&mut pheromon_data);
            self.add_pheromon(&ants, &mut pheromon_data);
            self.correct_pheromon(&mut pheromon_data);
            self.elite_boost(&mut pheromon_data, &best_t, best_l);
        }
        best_t.push(best_t[0]);
        (best_l, best_t)
    }

    fn add_pheromon(&self, ants: &[Ant], pdata: &mut [Vec<f64>]) {
        let q = self.q;
        ants.iter().for_each(|ant| {
            let (l, route) = ant.data();
            let val = q / l;
            for path in route.windows(2) {
                pdata[path[0]][path[1]] += val;
                pdata[path[1]][path[0]] += val;
            }
        })
    }

    fn correct_pheromon(&self, pdata: &mut [Vec<f64>]) {
        let low_bound = self.config.pheromon_min;
        pdata.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|v| {
                if *v < low_bound {
                    *v = low_bound
                }
            })
        });
    }

    fn evaporation(&self, pdata: &mut [Vec<f64>]) {
        let left = 1.0 - self.config.p;
        pdata
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|v| *v *= left))
    }

    fn elite_boost(&self, pdata: &mut [Vec<f64>], best_t: &[usize], best_l: f64) {
        let val = self.config.e as f64 * self.q / best_l;
        best_t.windows(2).for_each(|win| {
            pdata[win[0]][win[1]] += val;
            pdata[win[1]][win[0]] += val;
        });
        let (first, last) = (best_t[0], best_t[best_t.len() - 1]);
        pdata[first][last] += val;
        pdata[last][first] += val;
    }

    fn init_params(&self) -> (ThreadRng, Vec<Vec<f64>>, Vec<usize>, f64) {
        let rng = thread_rng();
        let pheromon_data =
            vec![vec![self.config.pheromon_start; self.ndata.len()]; self.ndata.len()];
        let best_t = (0..self.ndata.len()).collect::<Vec<usize>>();
        let best_l = best_t
            .windows(2)
            .fold(0.0, |acc, win| acc + 1.0 / self.ndata[win[0]][win[1]])
            + 1.0 / self.ndata[self.ndata.len() - 1][0];
        (rng, pheromon_data, best_t, best_l)
    }

    fn generate_ants(&self, rng: &mut ThreadRng) -> Vec<Ant> {
        let (m, data_len) = (self.config.m, self.ndata.len());
        let mut ants = (0..m - (m % data_len))
            .map(|start| Ant::new(data_len, start))
            .collect::<Vec<Ant>>();
        let more_ants = (0..(m % data_len))
            .map(|_| Ant::new(data_len, rng.gen_range(0, data_len)))
            .collect::<Vec<Ant>>();
        ants.extend_from_slice(&more_ants);
        ants
    }
}
