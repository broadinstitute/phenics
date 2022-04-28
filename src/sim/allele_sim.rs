use crate::phenotype::Phenotype;
use rand::prelude::{Distribution, ThreadRng};

pub(crate) struct AlleleSim {
    pub(crate) effects: Vec<f64>,
}

impl AlleleSim {
    pub(crate) fn from_phenotypes(phenotypes: &[Phenotype] ) -> AlleleSim {
        let mut effects: Vec<f64> = Vec::new();
        for phenotype in phenotypes {
            let effect =
                phenotype.sim.effect_distribution.sample::<ThreadRng>(&mut rand::thread_rng());
            effects.push(effect);
        }
        AlleleSim { effects }
    }
}