pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let mut seats = [false; 1024];
    println!(
        "Part 1: {}",
        input
            .lines()
            .map(|line| {
                let id = parse_seat_id(line.unwrap().as_str());
                seats[id] = true;
                id
            })
            .max()
            .unwrap()
    );

    let mut found_first = false;
    let mut missing = 0;
    for (seat, &exists) in seats.iter().enumerate() {
        found_first |= exists;
        if found_first && !exists {
            missing = seat;
            break;
        }
    }

    println!("Part 2: {}", missing);

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
