use crate::util::Unwrap;
use regex::Regex;
use std::borrow::Borrow;
use std::cmp::Ord;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::Range;
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
        .filter_map(|(field, &i)| {
            if field.field_name.starts_with("departure") {
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

    fn find_candidate_mappings(&self, field_id: usize) -> Vec<&Constraint> {
        let valid = self.valid_tickets();
        self.constraints
            .iter()
            .filter(|c| valid.iter().all(|t| c.matches(t[field_id])))
            .collect()
    }

    fn count_possible_assignments(&self, constraint: &Constraint) -> usize {
        self.find_candidate_assignments(constraint).len()
    }

    fn find_candidate_assignments(&self, constraint: &Constraint) -> Vec<usize> {
        let valid = self.valid_tickets();
        self.fields_iter()
            .filter(|&i| valid.iter().all(|t| constraint.matches(t[i])))
            .collect()
    }

    fn fields_iter(&self) -> Range<usize> {
        0..(self.my_ticket.len())
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

#[derive(Debug, Hash, PartialEq, Eq)]
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

fn find_mappings(problem: &Problem) -> HashMap<&Constraint, usize> {
    #[derive(PartialEq, Eq)]
    struct PendingConstraint<'a> {
        constraint: &'a Constraint,
        candidates: Vec<usize>,
    };

    impl PartialOrd for PendingConstraint<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            self.candidates.len().partial_cmp(&other.candidates.len())
        }
    }

    impl Ord for PendingConstraint<'_> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.candidates.len().cmp(&other.candidates.len())
        }
    }

    let mut constraints = BinaryHeap::from_iter(problem.constraints.iter().map(|c| {
        Reverse(PendingConstraint {
            constraint: c,
            candidates: problem.find_candidate_assignments(c),
        })
    }));

    let mut assigned_constraints = HashMap::new();
    let mut assigned_fields = HashSet::new();

    while let Some(c) = constraints.pop() {
        println!("Assigning {}, candidates={:?}", c.0.constraint.field_name, c.0.candidates);
        let remaining_candidates: Vec<&usize> =
            c.0.candidates
                .iter()
                .filter(|&i| !assigned_fields.contains(i))
                .collect();
        assert_eq!(remaining_candidates.len(), 1);
        let assignment = *remaining_candidates[0];
        assigned_constraints.insert(c.0.constraint, assignment);
        assigned_fields.insert(assignment);
    }
    assigned_constraints
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
        let mut mapping = find_mappings(&problem)
            .iter()
            .map(|(c, &i)| (c.field_name.as_str(), i))
            .collect::<Vec<(&str, usize)>>();
        mapping.sort_by(|(_, a), (_, b)| a.cmp(b));
        assert_eq!(mapping, vec![("row", 0), ("class", 1), ("seat", 2)]);
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
