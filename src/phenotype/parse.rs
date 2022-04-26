use crate::phenotype::Phenotype;
use crate::error::Error;
use std::num::ParseFloatError;

fn cannot_parse(string: &str, problem: &str) -> Error {
    Error::from(format!("Cannot parse {}: {}", string, problem))
}

pub(crate) fn parse(string: &str) -> Result<Phenotype, Error> {
    let mut split_by_eq = string.split('=');
    let name = String::from(split_by_eq.next().ok_or_else(|| {
        cannot_parse(string, "no '='")
    })?);
    let definition =
        split_by_eq.next().ok_or_else(|| { cannot_parse(string, "no '='") })?;
    if split_by_eq.next().is_some() {
        return Err(cannot_parse(string, "More than one '='."));
    }

    Ok(Phenotype::new(name))
}

enum Token {
    OpenParens,
    CloseParen,
    Comma,
    Number(f64),
    String(String),
}

fn min3(a: usize, b: usize, c: usize) -> usize {
    if a < b {
        if a < c { a } else { c }
    } else {
        if b < c { b } else { c }
    }
}

fn tokenize(string: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::<Token>::new();
    let mut remainder = string;
    while !remainder.is_empty() {
        let not_found = remainder.len();
        let open_pos = remainder.find('(').unwrap_or(not_found);
        if open_pos == 0usize {
            tokens.push(Token::OpenParens);
            remainder = &remainder[1..];
            break;
        }
        let comma_pos = remainder.find(',').unwrap_or(not_found);
        if comma_pos == 0usize {
            tokens.push(Token::Comma);
            remainder = &remainder[1..];
            break;
        }
        let close_pos = remainder.find(')').unwrap_or(not_found);
        if close_pos == 0usize {
            tokens.push(Token::CloseParen);
            remainder = &remainder[1..];
            break;
        }
        let min_pos = min3(open_pos, comma_pos, close_pos);
        let (first, last) = remainder.split_at(min_pos);
        remainder = last;
        match first.parse::<f64>() {
            Ok(number) => { tokens.push(Token::Number(number)) }
            Err(_) => { tokens.push(Token::String(String::from(first))) }
        }
    }
    Ok(tokens)
}