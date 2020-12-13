use rand::prelude::*;

mod config;
pub use config::Config;

mod ant;
use ant::Ant;

pub struct AntSolver {
    ndata: Vec<Vec<f64>>,
    q: f64,
    config: Config,
}

impl AntSolver {
    pub fn new(data: &[Vec<f64>], config: Config) -> Self {
        let ndata = data
            .iter()
            .map(|row| row.iter().map(|&e| 1_f64 / e).collect())
            .collect();
        let q = Self::compute_q(data);
        Self { ndata, q, config }
    }

    fn compute_q(data: &[Vec<f64>]) -> f64 {
        // sum of all
        let q = data
            .iter()
            .fold(0.0, |acc, row| acc + row.iter().sum::<f64>() as f64);
        // minus diagonal
        let q = q - (0..data.len()).fold(0.0, |acc, index| acc + data[index][index]);
        q / data.len() as f64
    }

    pub fn solve(&self) -> (f64, Vec<usize>) {
        let (mut rng, mut pheromon_data, mut best_t, mut best_l) = self.init_params();
        for _ in 0..self.config.tmax {
            let mut ants = self.generate_ants(&mut rng);
            // Run ants
            for a in ants.iter_mut() {
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

            // Update pheromon: evaporation, add_pheromon by ants,
            // correct_pheromon (min bound), elite_boost (add e * dt to best edges)
            self.evaporation(&mut pheromon_data);
            self.add_pheromon(&ants, &mut pheromon_data);
            self.correct_pheromon(&mut pheromon_data);
            self.elite_boost(&mut pheromon_data, &best_t, best_l);
        }

        best_t.push(best_t[0]);
        (best_l, best_t)
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
}
