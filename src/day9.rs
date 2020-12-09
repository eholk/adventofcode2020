use std::borrow::Borrow;

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let input = read_input(input.lines().map(|line| line.unwrap()));

    println!("Part 1: {}", find_invalid(input.as_slice(), 25).unwrap());

    Ok(())
}

fn read_input<Lines: Iterator>(lines: Lines) -> Vec<usize>
where
    Lines::Item: Borrow<str>,
{
    lines.map(|line| line.borrow().parse().unwrap()).collect()
}

fn is_valid(previous: &[usize], number: usize) -> bool {
    for i in 0..previous.len() {
        for j in i+1..previous.len() {
            if previous[i] != previous[j] && previous[i] + previous[j] == number {
                return true;
            }
        }
    }
    false
}

fn find_invalid(numbers: &[usize], window_size: usize) -> Option<usize> {
    for window in numbers.windows(window_size + 1) {
        let last = window[window_size];
        if !is_valid(&window[0..window_size], last) {
            return Some(last);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_slice() {
        assert!(is_valid(&[35, 20, 15, 25, 47], 40))
    }

    #[test]
    fn validate_example() {
        let example = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let sequence = read_input(example.lines());
        assert_eq!(find_invalid(sequence.as_slice(), 5), Some(127));
    }
}