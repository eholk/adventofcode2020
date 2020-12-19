use crate::util::Unwrap;
use regex::Regex;
use std::borrow::Borrow;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Constraint {
    field_name: String,
    ranges: [RangeInclusive<usize>; 2],
}

impl FromStr for Constraint {
    type Err = <usize as std::str::FromStr>::Err;

    fn from_str(s: &str) -> Result<Constraint, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<field>[\w ]+): (?P<low1>\d+)-(?P<high1>\d+) or (?P<low2>\d+)-(?P<high2>\d+)"
            )
            .unwrap();
        }
        if let Some(captures) = RE.captures(s) {
            Ok(Constraint {
                field_name: captures["field"].into(),
                ranges: [
                    (captures["low1"].parse()?)..=(captures["high1"].parse()?),
                    (captures["low2"].parse()?)..=(captures["high2"].parse()?),
                ],
            })
        } else {
            "unmatched regex".parse()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_constraint() {
        assert_eq!(
            "class: 1-3 or 5-7".parse::<Constraint>().unwrap(),
            Constraint {
                field_name: "class".into(),
                ranges: [1..=3, 5..=7],
            }
        );

        assert_eq!(
            "row: 6-11 or 33-44".parse::<Constraint>().unwrap(),
            Constraint {
                field_name: "row".into(),
                ranges: [6..=11, 33..=44],
            }
        );

        assert_eq!(
            "seat: 13-40 or 45-50".parse::<Constraint>().unwrap(),
            Constraint {
                field_name: "seat".into(),
                ranges: [13..=40, 45..=50],
            }
        );
    }
}
