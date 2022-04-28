use crate::phenotype::parse::treeize::{Call, Tree};
use crate::error::Error;
use crate::phenotype::pheno_sim::{PhenoSim, MyDistribution, Category};
use crate::phenotype::parse::Value;

pub(super) fn analyze(call: Call) -> Result<PhenoSim, Error> {
    if call.args.len() < 2 {
        return Err(Error::from(
            format!("Phenotype definition needs as least two arguments, but got {}",
                    call.args.len()))
        );
    }
    let mut args_iter = call.args.into_iter();
    let effect_distribution = distribution(args_iter.next().unwrap())?;
    let heritability = number(args_iter.next().unwrap())?;
    let category =
        args_iter.next().map(|tree| { category(tree) }).transpose()?
            .unwrap_or(Category::Quantitative);
    Ok(PhenoSim::new(effect_distribution, heritability, category))
}

fn distribution(tree: Tree) -> Result<MyDistribution, Error> {
    match tree {
        Tree::Call(call) => {
            match call.name.as_str() {
                "norm" => { norm(call) }
                "pick" => { pick(call) }
                _ => {
                    Err(Error::from(format!("`{}` is not recognized as a distribution.",
                                            call.name.as_str())))
                }
            }
        }
        Tree::Value(Value::Number(number)) => {
            Ok(MyDistribution::new_stuck(number))
        }
        Tree::Value(Value::String(string)) => {
            Err(Error::from(
                format!("Distribution needs to be a call or number, but got string `{}`.",
                        string)))
        }
    }
}

fn norm(call: Call) -> Result<MyDistribution, Error> {
    if call.args.len() != 2 {
        return Err(Error::from(format!("`norm` takes two arguments, but got {}.",
                                       call.args.len())));
    }
    let mut args_iter = call.args.into_iter();
    let mean = number(args_iter.next().unwrap())?;
    let std_dev = number(args_iter.next().unwrap())?;
    MyDistribution::new_normal(mean, std_dev)
}

fn pick(call: Call) -> Result<MyDistribution, Error> {
    if call.args.len() < 4 {
        return Err(Error::from(format!("`pick` takes at least four arguments, but got {}.",
                                       call.args.len())));
    }
    if call.args.len() % 2 == 1 {
        return Err(Error::from(
            format!("`pick` takes an even number of arguments, but got {}.",
                    call.args.len())));
    }
    let mut args_iter = call.args.into_iter();
    let mut weights: Vec<f64> = Vec::new();
    let mut distributions: Vec<MyDistribution> = Vec::new();
    while let Some(arg_odd) = args_iter.next() {
        weights.push(number(arg_odd)?);
        distributions.push(distribution(args_iter.next().unwrap())?);
    }
    MyDistribution::new_pick(weights, distributions)
}

fn number(tree: Tree) -> Result<f64, Error> {
    match tree {
        Tree::Value(Value::Number(number)) => { Ok(number) }
        _ => { Err(Error::from("Argument needs to be a number.")) }
    }
}

fn category(tree: Tree) -> Result<Category, Error> {
    match tree {
        Tree::Call(call) => {
            match call.name.as_str() {
                "bin" => { bin(call) }
                _ => {
                    Err(Error::from("The only category recognized is `bin`."))
                }
            }
        }
        _ => {
            Err(Error::from("Argument needs to be a category definition."))
        }
    }
}

fn bin(call: Call) -> Result<Category, Error> {
    if call.args.is_empty() {
        return Err(Error::from("Bin needs as least one argument"));
    }
    let mut args_iter = call.args.into_iter();
    let prevalence = number(args_iter.next().unwrap())?;
    let case =
        args_iter.next().map(|tree| { value(tree) }).transpose()?
            .map(|value| { format!("{}", value) }).unwrap_or_else(|| String::from("case"));
    let control =
        args_iter.next().map(|tree| { value(tree) }).transpose()?
            .map(|value| { format!("{}", value) }).unwrap_or_else(|| String::from("control"));
    Ok(Category::Binary(prevalence, case, control))
}

fn value(tree: Tree) -> Result<Value, Error> {
    match tree {
        Tree::Call(_) => { Err(Error::from("Argument needs to be a value.")) }
        Tree::Value(value) => { Ok(value) }
    }
}