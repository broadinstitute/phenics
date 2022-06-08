use crate::error::Error;
use noodles::core::Region;
use noodles::core::position::Position;

fn parse_error(string: &str) -> Error {
    Error::from(format!("Cannot parse '{}' as a region.", string))
}

pub(crate) fn parse(string: &str) -> Result<Region, Error> {
    let mut parts = string.split(':');
    let chrom = String::from(parts.next().ok_or_else(|| { parse_error(string) })?);
    let part2 = parts.next().ok_or_else(|| { parse_error(string) })?;
    let mut limits = part2.split('-');
    let start =
        Position::try_from(limits.next().ok_or_else(|| { parse_error(string) })?
            .parse::<usize>()?)?;
    let end =
        Position::try_from(limits.next().ok_or_else(|| { parse_error(string) })?
            .parse::<usize>()?)?;
    Ok(Region::new(chrom, start..=end))
}