use rand::distributions::{Distribution, WeightedIndex};
use rand_distr::Normal;
use rand::Rng;
use crate::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone)]
pub(crate) struct PhenoSim {
    pub(crate) effect_distribution: MyDistribution,
    pub(crate) heritability: f64,
    category: Category,
}

pub(crate) enum Category {
    Quantitative,
    Binary(Binary),
}

pub(crate) struct Binary {
    prevalence: f64,
    pub(crate) case: String,
    pub(crate) control: String,
}

#[derive(Clone)]
pub(crate) enum MyDistribution {
    Stuck(StuckDistribution),
    Norm(Normal<f64>),
    Pick(PickDistribution),
}

#[derive(Clone)]
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

impl Binary {
    pub(crate) fn new(prevalence: f64, case: String, control: String) -> Result<Binary, Error> {
        if prevalence < 0.0 || prevalence > 1.0 {
            Err(Error::from(
                format!("Prevalence needs to be between 0.0 and 1.0, but is {}.", prevalence)
            ))
        } else {
            Ok(Binary { prevalence, case, control })
        }
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

impl Clone for Binary {
    fn clone(&self) -> Self {
        let prevalence = self.prevalence;
        let case = self.case.clone();
        let control = self.control.clone();
        Binary { prevalence, case, control }
    }
}

impl Clone for Category {
    fn clone(&self) -> Self {
        match self {
            Category::Quantitative => { Category::Quantitative }
            Category::Binary(binary) => { Category::Binary(binary.clone()) }
        }
    }
}

impl Clone for PickDistribution {
    fn clone(&self) -> Self {
        let index_distribution = self.index_distribution.clone();
        let distributions = self.distributions.clone();
        PickDistribution { index_distribution, distributions }
    }
}

impl Display for PhenoSim {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.category {
            Category::Quantitative => {
                write!(f, "{},{}", self.effect_distribution, self.heritability)
            }
            Category::Binary(Binary { prevalence, case, control }) => {
                write!(f, "{},{},bin({},{},{})", self.effect_distribution, self.heritability,
                       prevalence, case, control)
            }
        }
    }
}

impl Display for MyDistribution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyDistribution::Stuck(stuck) => { write!(f, "{}", stuck.value) }
            MyDistribution::Norm(norm) => {
                write!(f, "norm({},{})", norm.mean(), norm.std_dev())
            }
            MyDistribution::Pick(pick) => { write!(f, "{}", pick) }
        }
    }
}

impl Display for PickDistribution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let args =
            self.distributions.iter().map(|dist| {
                format!("?,{}", dist)
            }).collect::<Vec<String>>().join(",");
        write!(f, "pick({})", args)
    }
}

