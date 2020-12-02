use regex::Regex;

struct Constraint {
    min_count: usize,
    max_count: usize,
    character: char,
}

impl Constraint {
    fn matches(&self, password: &str) -> bool {
        let mut count = 0;
        for c in password.chars() {
            if c == self.character {
                count += 1;
            }
        }
        self.min_count <= count && count <= self.max_count
    }

    // Returns whether the password matches the interpretation of the password rules from Part 2.
    fn matches2(&self, password: &str) -> bool {
        let mut count = 0;
        for (i, c) in password.char_indices() {
            if (i+1 == self.min_count || i+1 == self.max_count) && c == self.character {
                count += 1;
            }
        }

        count == 1
    }
}

fn parse_line(line: &str) -> (Constraint, String) {
    lazy_static! {
        static ref LINE_MATCHER: Regex =
            Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<char>\w): (?P<password>\w+)").unwrap();
    }

    let captures = LINE_MATCHER.captures(line).unwrap();
    let min = &captures["min"];
    let max = &captures["max"];
    let character = &captures["char"];
    let password = &captures["password"];

    (
        Constraint {
            min_count: min.parse().unwrap(),
            max_count: max.parse().unwrap(),
            character: character.chars().nth(0).unwrap(),
        },
        password.to_string(),
    )
}

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let mut valid_lines = 0;
    let mut valid_lines2 = 0;
    for line in input.lines() {
        let (constraint, password) = parse_line(line?.as_str());
        if constraint.matches(password.as_str()) {
            valid_lines += 1;
        }
        if constraint.matches2(password.as_str()) {
            valid_lines2 += 1;
        }
    }
    println!("Part 1: {}", valid_lines);
    println!("Part 2: {}", valid_lines2);
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_line() {
        let line = "1-3 a: abcde";
        let (constraint, password) = super::parse_line(line);
        assert_eq!(constraint.min_count, 1);
        assert_eq!(constraint.max_count, 3);
        assert_eq!(constraint.character, 'a');
        assert_eq!(password, "abcde");
    }

    #[test]
    fn test_matches() {
        let cases = [("1-3 a: abcde", true), ("1-3 b: cdefg", false), ("2-9 c: ccccccccc", true)];
        for (line, expected) in &cases {
            let (constraint, password) = super::parse_line(line);
            assert_eq!(constraint.matches(password.as_str()), *expected);
        }
    }

    #[test]
    fn test_matches2() {
        let cases = [("1-3 a: abcde", true), ("1-3 b: cdefg", false), ("2-9 c: ccccccccc", false)];
        for (line, expected) in &cases {
            let (constraint, password) = super::parse_line(line);
            assert_eq!(constraint.matches2(password.as_str()), *expected);
        }
    }
}
