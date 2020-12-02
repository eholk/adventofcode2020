use std::io;
use std::io::BufRead;

pub fn solve(expenses: &[usize]) -> usize {
    for i in 0..expenses.len() {
        for j in i+1..expenses.len() {
            if expenses[i] + expenses[j] == 2020 {
                return expenses[i] * expenses[j];
            }
        }
    }
    panic!("no solution found");
}

pub fn read_input<T: BufRead>(input: T) -> io::Result<Vec<usize>> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(line?.parse().unwrap());
    }
    return Ok(result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        assert!(solve(&[1721,
            979,        
            366,
            299,
            675,
            1456]) == 514579)
    }

    #[test]
    fn read_example() {
        let buffer = io::BufReader::new("1721
979
366
299
675
1456
".as_bytes());
        let result = solve(read_input(buffer).unwrap().as_slice());
        assert!(result == 514579);
    }
}
