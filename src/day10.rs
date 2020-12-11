use crate::day9::read_input;

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let mut input = read_input(input.lines().map(|line| line.unwrap()));
    input.sort();

    println!("Part 1: {}", part1(input.as_slice()));
    println!("Part 2: {}", part2(input.as_slice()));

    Ok(())
}

// input must be sorted
fn part1(data: &[usize]) -> usize {
    let mut counts = [0usize; 4];

    data.iter().fold(0, |a, b| {
        counts[b - a] += 1;
        *b
    });

    // Add 1 because of the last jump to my device's joltage.
    counts[1] * (counts[3] + 1)
}

// input must be sorted
fn part2(data: &[usize]) -> usize {
    let mut counts = vec![0usize; data.len()];

    let mut i: isize = data.len() as isize - 1;
    counts[data.len() - 1] = 1;
    i -= 1;
    while i >= 0 {
        let jolts = data[i as usize];
        for j in i as usize + 1..data.len() {
            if data[j] - jolts > 3 {
                break;
            }
            counts[i as usize] += counts[j];
        }
        i -= 1;
    }

    let mut total = 0;
    for j in 0..data.len() {
        if data[j] > 3 {
            return total;
        }
        total += counts[j];
    }

    panic!("this shouldn't happen");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let mut numbers = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        numbers.sort();

        assert_eq!(part1(&numbers), 220);
    }

    #[test]
    fn part2_example() {
        let mut numbers = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        numbers.sort();

        assert_eq!(part2(&numbers), 19208);
    }
}
