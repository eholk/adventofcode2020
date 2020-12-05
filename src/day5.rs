pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    println!(
        "Part 1: {}",
        input
            .lines()
            .map(|line| parse_seat_id(line.unwrap().as_str()))
            .max()
            .unwrap()
    );
    Ok(())
}

fn parse_seat_id(s: &str) -> usize {
    s.chars()
        .map(|c| match c {
            'F' | 'L' => 0,
            'B' | 'R' => 1,
            _ => panic!("unexpected character"),
        })
        .fold(0, |a, b| (a << 1) + b)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_seat_ids() {
        assert_eq!(parse_seat_id("FBFBBFFRLR"), 357);
        assert_eq!(parse_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(parse_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(parse_seat_id("BBFFBBFRLL"), 820);
    }
}
