use crate::error::Error;

pub(crate) struct Stats {
    sums: Vec<f64>,
    squares_sums: Vec<f64>,
}

impl Stats {
    pub(crate) fn new(n: usize) -> Stats {
        let sums: Vec<f64> = vec![0.0; n];
        let squares_sums: Vec<f64> = vec![0.0; n];
        Stats { sums, squares_sums }
    }
    pub(crate) fn add(&mut self, values: Vec<f64>) -> Result<(), Error> {
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
    pub(crate) fn means(&self) -> Vec<f64> {
        let n = self.sums.len() as f64;
        self.sums.iter().map(|sum| { sum / n }).collect()
    }
    pub(crate) fn sigmas(&self) -> Vec<f64> {
        let n = (self.sums.len() - 1) as f64;
        self.squares_sums.iter().enumerate().map(|(i, square_sum)| {
            let sum = self.sums[i];
            (square_sum - sum * sum) / n
        }).collect()
    }
}