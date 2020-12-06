use std::collections::HashSet;

pub fn run<IO: std::io::Read>(mut input: IO) -> std::io::Result<()> {
    let mut data = String::new();
    input.read_to_string(&mut data)?;

    println!("Part 1: {}", count_groups(data.as_str()));
    println!("Part 2: {}", count_groups_all(data.as_str()));

    Ok(())
}

fn count_groups(input: &str) -> usize {
    let mut count = 0;
    for group in input.split("\r\n\r\n") {
        let mut answers = std::collections::HashSet::new();
        for c in group.chars() {
            if 'a' <= c && c <= 'z' {
                answers.insert(c);
            }
        }
        count += answers.len()
    }
    count
}

fn count_groups_all(input: &str) -> usize {
    let mut count = 0;
    let all_answers = "abcdefghijklmnopqrstuvwxyz".chars().collect::<HashSet<char>>();
    for group in input.split("\r\n\r\n") {
        count += group.lines().map(|person| person.chars().collect::<HashSet<char>>())
        .fold(all_answers.clone(), |a, b| a.intersection(&b).map(|&a| a).collect()).len();
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_groups_example() {
        assert_eq!(count_groups(EXAMPLE_GROUPS), 11);
    }

    static EXAMPLE_GROUPS: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";
}