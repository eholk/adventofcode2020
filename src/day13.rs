pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let mut lines = input.lines();
    let start = lines.next().unwrap()?.parse::<usize>().unwrap();
    let busses: Vec<Option<usize>> = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|bus| {
            if bus == "x" {
                None
            } else {
                Some(bus.parse().unwrap())
            }
        })
        .collect();

    let (best_id, best_wait) = find_nearest(start, busses.as_slice());

    println!("Part 1: {}", best_id * best_wait);
    println!("Part 2: {}", find_consecutive(busses.as_slice()));

    Ok(())
}

// returns (id, wait_time)
fn find_nearest(start: usize, busses: &[Option<usize>]) -> (usize, usize) {
    busses
        .iter()
        .fold((0, usize::MAX), |(best_id, best_wait), bus| match bus {
            None => (best_id, best_wait),
            Some(id) => {
                let wait = (start + id - 1) / id * id - start;
                if wait < best_wait {
                    (*id, wait)
                } else {
                    (best_id, best_wait)
                }
            }
        })
}

fn find_consecutive(busses: &[Option<usize>]) -> usize {
    let step = busses[0].unwrap();
    let mut t = 100000000000000;
    t -= t % step;
    while !busses.iter().enumerate().all(|(offset, bus)| match bus {
        Some(freq) => (t + offset) % freq == 0,
        None => true,
    }) {
        t += step;
    }
    t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let start = 939;
        let busses = vec![Some(7), Some(13), Some(59), Some(31), Some(19)];

        let (best_id, best_wait) = find_nearest(start, busses.as_slice());
        assert_eq!(best_id * best_wait, 295);
    }

    #[test]
    fn consecutive_example() {
        let busses = vec![Some(7), Some(13), None, None, Some(59), None, Some(31), Some(19)];

        assert_eq!(find_consecutive(busses.as_slice()), 1068781);
    }
}
