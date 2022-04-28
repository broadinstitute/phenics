use rand::distributions::{Distribution, WeightedIndex};
use rand_distr::Normal;
use rand::Rng;
use crate::error::Error;

pub(crate) struct PhenoSim {
    pub(crate) effect_distribution: MyDistribution,
    heritability: f64,
    category: Category,
}

pub(crate) enum Category {
    Quantitative,
    Binary(f64, String, String),
}

pub(crate) enum MyDistribution {
    Stuck(StuckDistribution),
    Norm(Normal<f64>),
    Pick(PickDistribution),
}

pub(crate) struct StuckDistribution {
    value: f64,
}

pub(crate) struct PickDistribution {
    index_distribution: WeightedIndex<f64>,
    distributions: Vec<MyDistribution>,
}

impl PhenoSim {
    pub(crate) fn new(effect_distribution: MyDistribution, heritability: f64, category: Category)
        -> PhenoSim {
        PhenoSim { effect_distribution, heritability, category }
    }
}

impl StuckDistribution {
    pub(crate) fn new(value: f64) -> StuckDistribution {
        StuckDistribution { value }
    }
}

impl Distribution<f64> for StuckDistribution {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> f64 {
        self.value
    }
}

impl PickDistribution {
    pub(crate) fn new(weights: Vec<f64>, distributions: Vec<MyDistribution>)
                      -> Result<PickDistribution, Error> {
        let index_distribution = WeightedIndex::new(weights)?;
        Ok(PickDistribution { index_distribution, distributions })
    }
}

impl Distribution<f64> for PickDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        self.distributions[self.index_distribution.sample(rng)].sample(rng)
    }
}

impl MyDistribution {
    pub(crate) fn new_stuck(value: f64) -> MyDistribution {
        MyDistribution::Stuck(StuckDistribution::new(value))
    }
    pub(crate) fn new_normal(mean: f64, std_dev: f64) -> Result<MyDistribution, Error> {
        Ok(MyDistribution::Norm(Normal::new(mean, std_dev)?))
    }
    pub(crate) fn new_pick(weights: Vec<f64>, distributions: Vec<MyDistribution>)
        -> Result<MyDistribution, Error> {
        Ok(MyDistribution::Pick(PickDistribution::new(weights, distributions)?))
    }
}

impl Distribution<f64> for MyDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        match self {
            MyDistribution::Stuck(stuck) => { stuck.sample(rng) }
            MyDistribution::Norm(norm) => { norm.sample(rng) }
            MyDistribution::Pick(pick) => { pick.sample(rng) }
        }
    }
}

