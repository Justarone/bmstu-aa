use super::Cost;
use rand::prelude::*;

mod config;
pub use config::Config;

mod ant;
use ant::Ant;

pub struct AntSolver<'a> {
    data: &'a [Vec<Cost>],
    ndata: Vec<Vec<f64>>,
    q: f64,
    config: Config,
}

impl<'a> AntSolver<'a> {
    pub fn new(data: &'a [Vec<Cost>], config: Config) -> Self {
        let ndata = data
            .iter()
            .map(|row| row.iter().map(|&e| 1_f64 / e as f64).collect())
            .collect();
        let q = Self::compute_q(data);
        Self {
            data,
            ndata,
            q,
            config,
        }
    }

    fn compute_q(data: &[Vec<Cost>]) -> f64 {
        // sum of all
        let q = data
            .iter()
            .fold(0 as Cost, |acc, row| acc + row.iter().sum::<Cost>()) as f64;
        q / data.len() as f64
    }

    pub fn solve(&self) -> (Cost, Vec<usize>) {
        let (mut rng, mut pheromon_data, mut best_t, mut best_l) = self.init_params();
        for _ in 0..self.config.tmax {
            let mut ants = self.generate_ants(&mut rng);
            // Run ants
            for a in ants.iter_mut() {
                a.walk(
                    self.data,
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
                .min_by(|a, b| a.data().0.cmp(&b.data().0))
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

    fn init_params(&self) -> (ThreadRng, Vec<Vec<f64>>, Vec<usize>, Cost) {
        let rng = thread_rng();
        let pheromon_data =
            vec![vec![self.config.pheromon_start; self.data.len()]; self.data.len()];
        let best_t = (0..self.data.len()).collect::<Vec<usize>>();
        let best_l = best_t
            .windows(2)
            .fold(0 as Cost, |acc, win| acc + self.data[win[0]][win[1]])
            + self.data[self.data.len() - 1][0];
        (rng, pheromon_data, best_t, best_l)
    }

    fn generate_ants(&self, rng: &mut ThreadRng) -> Vec<Ant> {
        let (m, data_len) = (self.config.m, self.data.len());
        // placing ants in all places works worse
        let ants = (0..m)
            .map(|_| Ant::new(data_len, rng.gen_range(0, data_len)))
            .collect::<Vec<Ant>>();
        ants
    }

    fn add_pheromon(&self, ants: &[Ant], pdata: &mut [Vec<f64>]) {
        let q = self.q;
        ants.iter().for_each(|ant| {
            let (l, route) = ant.data();
            let val = q / l as f64;
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

    fn elite_boost(&self, pdata: &mut [Vec<f64>], best_t: &[usize], best_l: Cost) {
        let val = self.config.e as f64 * self.q / best_l as f64;
        best_t.windows(2).for_each(|win| {
            pdata[win[0]][win[1]] += val;
            pdata[win[1]][win[0]] += val;
        });
        let (first, last) = (best_t[0], best_t[best_t.len() - 1]);
        pdata[first][last] += val;
        pdata[last][first] += val;
    }
}
