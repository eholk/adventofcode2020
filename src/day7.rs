use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::iter::{FromIterator, Iterator};

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let graph = parse_graph(input.lines().map(|line| line.unwrap()));

    println!("Part 1: {}", part1(&graph));
    Ok(())
}

fn parse_graph<Lines: Iterator>(lines: Lines) -> Graph where Lines::Item: Borrow<str> {
    lines.map(|line| parse_line(line.borrow())).collect()
}

fn part1(graph: &Graph) -> usize {
    // build the reverse graph
    let mut backedges = HashMap::<String, HashSet<String>>::new();
    for node in graph {
        for (_, edge) in &node.edges {
            match backedges.get_mut(edge) {
                Some(edges) => edges.insert(node.name.clone()),
                None => backedges
                    .insert(
                        edge.clone(),
                        HashSet::from_iter([&node.name].iter().map(|&s| s.clone())),
                    )
                    .is_some(),
            };
        }
    }

    fn traverse<'a>(
        backedges: &'a HashMap<String, HashSet<String>>,
        edge: &String,
    ) -> HashSet<&'a String> {
        match backedges.get(edge) {
            None => HashSet::new(),
            Some(edges) => edges
                .iter()
                .map(|edge| traverse(backedges, edge))
                .fold(HashSet::from_iter(edges), |a, b| a.union(&b).map(|&x| x).collect()),
        }
    }

    let transitive_closure = traverse(&backedges, &"shiny gold".to_string());
    transitive_closure.len()
}

type Graph = Vec<Node>;

#[derive(Debug, PartialEq)]
struct Node {
    name: String,
    edges: Vec<(usize, String)>,
}

fn parse_line(line: &str) -> Node {
    let mut parts = line.split(" bags contain ");
    let name = parts.next().unwrap();
    let contents = parts.next().unwrap();
    if contents == "no other bags." {
        Node {
            name: name.to_string(),
            edges: vec![],
        }
    } else {
        Node {
            name: name.to_string(),
            edges: contents
                .split(", ")
                .map(|entry| {
                    let mut parts = entry.split(" ");
                    let count = parts.next().unwrap();
                    let name = parts.next().unwrap().to_string() + " " + parts.next().unwrap();
                    (count.parse().unwrap(), name)
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_two_bags() {
        let line = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        assert_eq!(
            parse_line(line),
            Node {
                name: "light red".to_string(),
                edges: vec![
                    (1, "bright white".to_string()),
                    (2, "muted yellow".to_string())
                ]
            }
        );
    }

    #[test]
    fn parse_line_no_bags() {
        let line = "dotted black bags contain no other bags.";
        assert_eq!(
            parse_line(line),
            Node {
                name: "dotted black".to_string(),
                edges: vec![]
            }
        );
    }

    #[test]
    fn parse_line_one_bag() {
        let line = "bright white bags contain 1 shiny gold bag.";
        assert_eq!(
            parse_line(line),
            Node {
                name: "bright white".to_string(),
                edges: vec![(1, "shiny gold".to_string())]
            }
        );
    }

    #[test]
    fn traverse_example() {
        let graph = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        let graph = parse_graph(graph.lines());

        assert_eq!(part1(&graph), 4);
    }
}
