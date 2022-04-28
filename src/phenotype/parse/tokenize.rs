use crate::phenotype::parse::Value;
use crate::error::Error;

pub(super) enum Token {
    OpenParens,
    CloseParen,
    Comma,
    Value(Value),
}

fn min3(a: usize, b: usize, c: usize) -> usize {
    if a < b {
        if a < c { a } else { c }
    } else if b < c { b } else { c }

}

pub(super) fn tokenize(string: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::<Token>::new();
    let mut remainder = String::from(string);
    while !remainder.is_empty() {
        let not_found = remainder.len();
        let open_pos = remainder.find('(').unwrap_or(not_found);
        if open_pos == 0usize {
            tokens.push(Token::OpenParens);
            remainder.remove(0);
            break;
        }
        let comma_pos = remainder.find(',').unwrap_or(not_found);
        if comma_pos == 0usize {
            tokens.push(Token::Comma);
            remainder.remove(0);
            break;
        }
        let close_pos = remainder.find(')').unwrap_or(not_found);
        if close_pos == 0usize {
            tokens.push(Token::CloseParen);
            remainder.remove(0);
            break;
        }
        let min_pos = min3(open_pos, comma_pos, close_pos);
        let prefix_range = ..min_pos;
        let prefix = &remainder[prefix_range];
        let value = match prefix.parse::<f64>() {
            Ok(number) => { Value::Number(number) }
            Err(_) => { Value::String(String::from(prefix)) }
        };
        remainder.replace_range(prefix_range, "");
        tokens.push(Token::Value(value))
    }
    Ok(tokens)
}

