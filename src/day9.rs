use std::borrow::Borrow;

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let input = read_input(input.lines().map(|line| line.unwrap()));

    let invalid = find_invalid(input.as_slice(), 25).unwrap();
    println!("Part 1: {}", invalid);
    println!("Part 2: {}", part2(input.as_slice(), invalid));

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
        for j in i + 1..previous.len() {
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

fn find_segment(numbers: &[usize], target: usize) -> &[usize] {
    for i in 0..numbers.len() {
        let mut sum = numbers[i];
        for j in i + 1..numbers.len() {
            sum += numbers[j];
            if sum == target {
                return &numbers[i..j + 1];
            }

            if sum > target {
                break;
            }
        }
    }

    panic!("no solution found");
}

fn part2(numbers: &[usize], target: usize) -> usize {
    let sequence = find_segment(numbers, target);
    sequence.iter().min().unwrap() + sequence.iter().max().unwrap()
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

    #[test]
    fn find_segment_example() {
        let numbers = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(find_segment(&numbers, 127), &[15, 25, 47, 40]);
        assert_eq!(part2(&numbers, 127), 62);
    }
}
