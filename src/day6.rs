pub fn run<IO: std::io::Read>(mut input: IO) -> std::io::Result<()> {
    let mut data = String::new();
    input.read_to_string(&mut data)?;

    println!("Part 1: {}", count_groups(data.as_str()));

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