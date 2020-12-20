use crate::util::Unwrap;
use regex::Regex;
use std::borrow::Borrow;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let problem = Problem::parse::<_, String>(input.lines());

    println!("Part 1: {}", part1(&problem));
    println!("Part 2: {}", part2(&problem));

    Ok(())
}

fn part1(problem: &Problem) -> usize {
    problem
        .nearby_tickets
        .iter()
        .map(|t| {
            t.iter()
                .filter(|&&i| !problem.is_valid_for_any_field(i))
                .sum::<usize>()
        })
        .sum()
}

fn part2(problem: &Problem) -> usize {
    let mappings = find_mappings(problem);

    mappings
        .iter()
        .filter_map(|&(field, i)| {
            if field.starts_with("departure") {
                Some(problem.my_ticket[i])
            } else {
                None
            }
        })
        .sum()
}

struct Problem {
    constraints: Vec<Constraint>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl Problem {
    fn all_tickets(
        &self,
    ) -> std::iter::Chain<
        std::iter::Once<&std::vec::Vec<usize>>,
        std::slice::Iter<std::vec::Vec<usize>>,
    > {
        let iter = std::iter::once(&self.my_ticket).chain(self.nearby_tickets.iter());
        iter
    }

    fn valid_tickets(&self) -> Vec<&Vec<usize>> {
        self.all_tickets()
            .filter(|t| t.iter().any(|&i| self.is_valid_for_any_field(i)))
            .collect()
    }

    fn is_valid_for_any_field(&self, i: usize) -> bool {
        self.constraints.iter().any(|c| c.matches(i))
    }

    fn parse<T: Iterator, B: Borrow<str>>(mut lines: T) -> Problem
    where
        T::Item: Unwrap<B>,
    {
        let constraints = parse_constraints(&mut lines);

        while let Some(line) = lines.next() {
            if line.unwrap().borrow() == "your ticket:" {
                break;
            }
        }
        let my_ticket = lines
            .next()
            .unwrap()
            .unwrap()
            .borrow()
            .split(",")
            .map(|i| i.parse().unwrap())
            .collect();
        while let Some(line) = lines.next() {
            if line.unwrap().borrow() == "nearby tickets:" {
                break;
            }
        }
        let nearby_tickets = lines
            .map(|line| {
                line.unwrap()
                    .borrow()
                    .split(",")
                    .map(|i| i.parse().unwrap())
                    .collect()
            })
            .collect();

        Problem {
            constraints,
            my_ticket,
            nearby_tickets,
        }
    }
}

fn parse_constraints<T: Iterator, S: Borrow<str>>(mut lines: T) -> Vec<Constraint>
where
    T::Item: Unwrap<S>,
{
    let mut constraints = Vec::new();
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.borrow().is_empty() {
            return constraints;
        }
        constraints.push(line.borrow().parse().unwrap())
    }
    constraints
}

#[derive(Debug, PartialEq)]
struct Constraint {
    field_name: String,
    ranges: [RangeInclusive<usize>; 2],
}

impl Constraint {
    fn matches(&self, i: usize) -> bool {
        self.ranges.iter().any(|r| r.contains(&i))
    }
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

fn find_mappings(problem: &Problem) -> Vec<(&str, usize)> {
    let valid_tickets = problem.valid_tickets();

    (0..(valid_tickets[0].len()))
        .map(|i| {
            println!("Finding field {}", i);
            problem.constraints.iter().find_map(|c| {
                println!("looking for {}", c.field_name);
                if c.field_name.starts_with("departure")
                    && valid_tickets.iter().all(|t| {
                        println!("testing {} of {:?}", t[i], t);
                        c.matches(t[i])
                    })
                {
                    Some((c.field_name.as_str(), i))
                } else {
                    None
                }
            })
        })
        .filter_map(|m| m)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let problem = Problem::parse(EXAMPLE_INPUT.lines());
        assert_eq!(part1(&problem), 71);
    }

    #[test]
    fn example_mapping() {
        let problem = Problem::parse(EXAMPLE_INPUT_2.lines());
        assert_eq!(
            find_mappings(&problem),
            vec![("row", 0), ("class", 1), ("seat", 2)]
        );
    }

    const EXAMPLE_INPUT: &str = &"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    const EXAMPLE_INPUT_2: &str = &"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

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

    #[test]
    fn parse_example_constraints() {
        assert_eq!(
            parse_constraints(EXAMPLE_INPUT.lines()),
            vec![
                Constraint {
                    field_name: "class".into(),
                    ranges: [1..=3, 5..=7],
                },
                Constraint {
                    field_name: "row".into(),
                    ranges: [6..=11, 33..=44],
                },
                Constraint {
                    field_name: "seat".into(),
                    ranges: [13..=40, 45..=50],
                }
            ]
        );
    }
}
