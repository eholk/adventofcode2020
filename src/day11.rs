use std::borrow::Borrow;

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let floorplan = parse_board(input.lines().map(|line| line.unwrap()));
    let floorplan = to_fixpoint(floorplan);

    println!("Part 1: {}", floorplan.count_occupied());

    Ok(())
}

fn parse_board<Lines: Iterator>(lines: Lines) -> FloorPlan
where
    Lines::Item: Borrow<str>,
{
    let seats = lines
        .map(|line| {
            line.borrow()
                .chars()
                .map(|c| match c {
                    'L' => Seat::Empty,
                    '.' => Seat::Floor,
                    '#' => Seat::Occupied,
                    other => panic!("Illegal character: {}", other),
                })
                .collect()
        })
        .collect();

    FloorPlan { seats }
}

enum Seat {
    Floor,
    Empty,
    Occupied,
}

fn to_fixpoint(mut floorplan: FloorPlan) -> FloorPlan {
    let mut changed = true;
    while changed {
        let (floorplan_, changed_) = floorplan.step();
        changed = changed_;
        floorplan = floorplan_;
    }
    floorplan
}

struct FloorPlan {
    seats: Vec<Vec<Seat>>,
}

impl FloorPlan {
    fn step(&self) -> (FloorPlan, bool) {
        let mut changed = false;
        let seats = (0..self.seats.len() as isize)
            .map(|i| {
                (0..self.seats[0].len() as isize)
                    .map(|j| match self[(i, j)] {
                        Seat::Floor => Seat::Floor,
                        Seat::Empty => {
                            if self.count_neighbors(i, j) == 0 {
                                changed = true;
                                Seat::Occupied
                            } else {
                                Seat::Empty
                            }
                        }
                        Seat::Occupied => {
                            if self.count_neighbors(i, j) >= 4 {
                                changed = true;
                                Seat::Empty
                            } else {
                                Seat::Occupied
                            }
                        }
                    })
                    .collect()
            })
            .collect();

        (FloorPlan { seats }, changed)
    }

    fn count_neighbors(&self, i: isize, j: isize) -> usize {
        (-1isize..=1)
            .map(|di| {
                (-1isize..=1)
                    .map(|dj| {
                        if di == 0 && dj == 0 {
                            0
                        } else {
                            match self[(i + di, j + dj)] {
                                Seat::Floor | Seat::Empty => 0,
                                Seat::Occupied => 1,
                            }
                        }
                    })
                    .fold(0, |a, b| a + b)
            })
            .fold(0, |a, b| a + b)
    }

    fn count_occupied(&self) -> usize {
        self.seats
            .iter()
            .map(|row| {
                row.iter()
                    .map(|seat| match seat {
                        Seat::Occupied => 1,
                        Seat::Empty | Seat::Floor => 0,
                    })
                    .fold(0, |a, b| a + b)
            })
            .fold(0, |a, b| a + b)
    }
}

impl std::fmt::Display for FloorPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.seats.as_slice() {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Seat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Seat::Floor => write!(f, "."),
            Seat::Occupied => write!(f, "#"),
            Seat::Empty => write!(f, "L"),
        }
    }
}

impl std::ops::Index<(isize, isize)> for FloorPlan {
    type Output = Seat;

    fn index(&self, (i, j): (isize, isize)) -> &Seat {
        if i < 0 || j < 0 || i as usize >= self.seats.len() || j as usize >= self.seats[0].len() {
            &Seat::Floor
        } else {
            &self.seats[i as usize][j as usize]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trace_example() {
        let example = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let floorplan = parse_board(example.lines());
        assert_eq!(to_fixpoint(floorplan).count_occupied(), 37);
    }
}
