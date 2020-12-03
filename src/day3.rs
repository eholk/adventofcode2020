pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let map = parse_map(input)?;

    println!("Part 1: {}", count_trajectory(&map, 1, 3));

    Ok(())
}

fn parse_line(line: &str) -> Vec<bool> {
    let mut result = Vec::new();

    for c in line.chars() {
        result.push(match c {
            '.' => false,
            '#' => true,
            _ => panic!("unsupported character"),
        })
    }

    result
}

struct Map {
    map: Vec<Vec<bool>>,
}

impl std::ops::Index<(usize, usize)> for Map {
    type Output = bool;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.map[i][j % self.map[i].len()]
    }
}

fn parse_map<IO: std::io::BufRead>(input: IO) -> std::io::Result<Map> {
    let mut map = Vec::new();
    for line in input.lines() {
        map.push(parse_line(line?.as_str()));
    }
    Ok(Map { map })
}

fn count_trajectory(map: &Map, di: usize, dj: usize) -> usize {
    let (mut i, mut j) = (0, 0);
    let mut count = 0;
    while i < map.map.len() {
        if map[(i, j)] {
            count += 1
        }
        i += di;
        j += dj;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_one_line() {
        assert_eq!(
            parse_line("..##......."),
            vec![false, false, true, true, false, false, false, false, false, false, false]
        );
    }

    #[test]
    fn count_trajectory_example() -> std::io::Result<()> {
        let map = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";
        let map = parse_map(std::io::BufReader::new(map.as_bytes()))?;
        assert_eq!(count_trajectory(&map, 1, 3), 7);
        Ok(())
    }
}
