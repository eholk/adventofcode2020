use std::borrow::Borrow;

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let floorplan = parse_board(input.lines().map(|line| line.unwrap()));
    let fixpoint = to_fixpoint(&floorplan, |floorplan| floorplan.step());

    println!("Part 1: {}", fixpoint.count_occupied());

    let fixpoint = to_fixpoint(&floorplan, |floorplan| floorplan.step2());
    println!("Part 2: {}", fixpoint.count_occupied());

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

#[derive(PartialEq, Clone, Copy)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

fn to_fixpoint<F>(floorplan: &FloorPlan, step: F) -> FloorPlan
where
    F: Fn(&FloorPlan) -> (FloorPlan, bool),
{
    let mut changed = true;
    let mut floorplan = floorplan;
    let mut floorplan_storage;
    while changed {
        let (floorplan_, changed_) = step(floorplan);
        if !changed_ {
            return floorplan_;
        }
        changed = changed_;
        floorplan_storage = floorplan_;
        floorplan = &floorplan_storage;
    }
    unreachable!();
}

struct FloorPlan {
    seats: Vec<Vec<Seat>>,
}

impl FloorPlan {
    fn step_by<F>(&self, step: F) -> (FloorPlan, bool)
    where
        F: Fn(isize, isize) -> Seat,
    {
        let mut changed = false;
        let seats = (0..self.seats.len() as isize)
            .map(|i| {
                (0..self.seats[0].len() as isize)
                    .map(|j| {
                        let seat = self[(i, j)];
                        let new_seat = step(i, j);
                        if seat != new_seat {
                            changed = true;
                        }
                        new_seat
                    })
                    .collect()
            })
            .collect();

        (FloorPlan { seats }, changed)
    }

    fn step(&self) -> (FloorPlan, bool) {
        self.step_by(|i, j| match self[(i, j)] {
            Seat::Floor => Seat::Floor,
            Seat::Empty => {
                if self.count_neighbors(i, j) == 0 {
                    Seat::Occupied
                } else {
                    Seat::Empty
                }
            }
            Seat::Occupied => {
                if self.count_neighbors(i, j) >= 4 {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
        })
    }

    fn step2(&self) -> (FloorPlan, bool) {
        self.step_by(|i, j| match self[(i, j)] {
            Seat::Floor => Seat::Floor,
            Seat::Empty => {
                if self.count_visible_neighbors(i, j) == 0 {
                    Seat::Occupied
                } else {
                    Seat::Empty
                }
            }
            Seat::Occupied => {
                if self.count_visible_neighbors(i, j) >= 5 {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
        })
    }

    fn look(&self, (mut i, mut j): (isize, isize), (di, dj): (isize, isize)) -> usize {
        i += di;
        j += dj;

        while self.inbounds(i, j) {
            match self[(i, j)] {
                Seat::Occupied => return 1,
                Seat::Empty => return 0,
                Seat::Floor => {
                    i += di;
                    j += dj;
                }
            }
        }
        0
    }

    fn inbounds(&self, i: isize, j: isize) -> bool {
        !(i < 0 || j < 0 || i as usize >= self.seats.len() || j as usize >= self.seats[0].len())
    }

    fn count_neighbors(&self, i: isize, j: isize) -> usize {
        self.count_neighbors_by(i, j, |(i, j), (di, dj)| match self[(i + di, j + dj)] {
            Seat::Floor | Seat::Empty => 0,
            Seat::Occupied => 1,
        })
    }

    fn count_visible_neighbors(&self, i: isize, j: isize) -> usize {
        self.count_neighbors_by(i, j, |x, dx| self.look(x, dx))
    }

    fn count_neighbors_by<F>(&self, i: isize, j: isize, count: F) -> usize
    where
        F: Fn((isize, isize), (isize, isize)) -> usize,
    {
        (-1isize..=1)
            .map(|di| {
                (-1isize..=1)
                    .map(|dj| {
                        if di == 0 && dj == 0 {
                            0
                        } else {
                            count((i, j), (di, dj))
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
        if !self.inbounds(i, j) {
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
        assert_eq!(
            to_fixpoint(&floorplan, |floorplan| floorplan.step()).count_occupied(),
            37
        );
    }

    #[test]
    fn trace_example2() {
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
        assert_eq!(
            to_fixpoint(&floorplan, |floorplan| floorplan.step2()).count_occupied(),
            26
        );
    }
}
