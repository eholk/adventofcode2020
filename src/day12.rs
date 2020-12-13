use std::borrow::Borrow;

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let input = parse_directions(input.lines().map(|line| line.unwrap()));

    let (n, e) = walk(input.as_slice());
    println!("Part 1: {}", n.abs() + e.abs());

    let (n, e) = walk_waypoint(input.as_slice());
    println!("Part 1: {}", n.abs() + e.abs());

    Ok(())
}

fn walk(directions: &[(char, isize)]) -> (isize, isize) {
    let (n, e, _dn, _de) = directions
        .iter()
        .fold((0, 0, 0, 1), |(n, e, dn, de), (dir, amount)| match dir {
            'F' => (n + dn * amount, e + de * amount, dn, de),
            'N' => (n + amount, e, dn, de),
            'S' => (n - amount, e, dn, de),
            'E' => (n, e + amount, dn, de),
            'W' => (n, e - amount, dn, de),
            'R' => {
                let (dn, de) = rotate(*amount, dn, de);
                (n, e, dn, de)
            }
            'L' => {
                let (dn, de) = rotate(-amount, dn, de);
                (n, e, dn, de)
            }
            other => panic!("Illegal direction: {}", other),
        });
    (n, e)
}

fn walk_waypoint(directions: &[(char, isize)]) -> (isize, isize) {
    let (n, e, _dn, _de) =
        directions
            .iter()
            .fold((0, 0, 1, 10), |(n, e, dn, de), (dir, amount)| match dir {
                'F' => (n + dn * amount, e + de * amount, dn, de),
                'N' => (n, e, dn + amount, de),
                'S' => (n, e, dn - amount, de),
                'E' => (n, e, dn, de + amount),
                'W' => (n, e, dn, de - amount),
                'R' => {
                    let (dn, de) = rotate(*amount, dn, de);
                    (n, e, dn, de)
                }
                'L' => {
                    let (dn, de) = rotate(-amount, dn, de);
                    (n, e, dn, de)
                }
                other => panic!("Illegal direction: {}", other),
            });
    (n, e)
}

fn rotate(dir: isize, n: isize, e: isize) -> (isize, isize) {
    if dir == 90 || dir == -90 {
        let sign = dir.signum();
        (-sign * e, sign * n)
    } else {
        if dir > 0 {
            let (n, e) = rotate(dir - 90, n, e);
            rotate(90, n, e)
        } else {
            let (n, e) = rotate(dir + 90, n, e);
            rotate(-90, n, e)
        }
    }
}

fn parse_directions<Lines: Iterator>(lines: Lines) -> Vec<(char, isize)>
where
    Lines::Item: Borrow<str>,
{
    lines.map(|line| parse_line(line.borrow())).collect()
}

fn parse_line(line: &str) -> (char, isize) {
    (
        line.chars().nth(0).unwrap(),
        line.split_at(1).1.parse().unwrap(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_lines() {
        assert_eq!(parse_line("F10"), ('F', 10));
        assert_eq!(parse_line("N3"), ('N', 3));
    }

    #[test]
    fn walk_example() {
        let directions = "F10\nN3\nF7\nR90\nF11";
        let directions = parse_directions(directions.lines());
        let (n, e) = walk(directions.as_slice());
        assert_eq!(n.abs() + e.abs(), 25);
    }

    #[test]
    fn walk_waypoint_example() {
        let directions = "F10\nN3\nF7\nR90\nF11";
        let directions = parse_directions(directions.lines());
        let (n, e) = walk_waypoint(directions.as_slice());
        assert_eq!(n.abs() + e.abs(), 286);
    }
}
