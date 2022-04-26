pub(crate) mod parse;

pub(crate) struct Phenotype {
    name: String
}

impl Phenotype {
    pub(crate) fn new(name: String) -> Phenotype {
        Phenotype { name }
    }
}