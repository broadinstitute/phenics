use crate::error::Error;

pub(crate) struct Stats {
    n: u64,
    sums: Vec<f64>,
    squares_sums: Vec<f64>,
}

impl Stats {
    pub(crate) fn new(n_phenotypes: usize) -> Stats {
        let n= 0u64;
        let sums: Vec<f64> = vec![0.0; n_phenotypes];
        let squares_sums: Vec<f64> = vec![0.0; n_phenotypes];
        Stats { n, sums, squares_sums }
    }
    pub(crate) fn add(&mut self, values: &[f64]) -> Result<(), Error> {
        self.n += 1;
        if values.len() != self.sums.len() {
            return Err(Error::from(
                format!("Need {} values, but got {}.", self.sums.len(), values.len())
            ));
        }
        for (i, value) in values.iter().enumerate() {
            self.sums[i] += value;
            self.squares_sums[i] += value * value;
        }
        Ok(())
    }
    pub(crate) fn variances(&self) -> Vec<f64> {
        let n = self.n as f64;
        self.squares_sums.iter().enumerate().map(|(i, square_sum)| {
            let sum = self.sums[i];
            let mean = sum / n;
            let mean_of_squares = square_sum / n;
            mean_of_squares - mean * mean
        }).collect()
    }
}