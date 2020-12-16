use std::collections::HashMap;
use std::iter::FromIterator;

const INPUT: &[usize] = &[6, 13, 1, 15, 2, 0];

pub fn run() {
    let last_spoken = play_game(INPUT, 2020);
    println!("Part 1: {}", last_spoken);

    println!("Part 2: {}", play_game(INPUT, 30000000));
}

struct Game {
    // Maps a number to the turn it was last spoken on, or None if it is new.
    last_spoken_on_turn: HashMap<usize, usize>,
    // the current turn
    turn: usize,
    // the number that will be spoken on this turn
    spoken: usize,
}

impl Game {
    fn new(init: &[usize]) -> Game {
        Game {
            turn: init.len(),
            last_spoken_on_turn: HashMap::from_iter(
                init.iter().enumerate().map(|(i, number)| (*number, i + 1)),
            ),
            spoken: init[init.len() - 1],
        }
    }

    fn take_turn(&mut self) {
        // it is currently turn self.turn, and we are going to say self.spoken
        //
        // We need to compute what we will say in turn+1.

        match self.last_spoken_on_turn.insert(self.spoken, self.turn) {
            None => self.spoken = 0,
            Some(turn) => self.spoken = self.turn - turn,
        }

        self.turn += 1;
    }
}

fn play_game(input: &[usize], turns: usize) -> usize {
    if turns - 1 < input.len() {
        input[turns - 1]
    } else {
        let mut game = Game::new(input);
        while game.turn < turns {
            game.take_turn();
        }
        game.spoken
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn play_game1() {
        let game = &[0, 3, 6];
        assert_eq!(play_game(game, 1), 0);
        assert_eq!(play_game(game, 2), 3);
        assert_eq!(play_game(game, 3), 6);
        assert_eq!(play_game(game, 4), 0);
        assert_eq!(play_game(game, 5), 3);
        assert_eq!(play_game(game, 7), 1);
        assert_eq!(play_game(game, 8), 0);
        assert_eq!(play_game(game, 9), 4);
        assert_eq!(play_game(game, 10), 0);
    }
}
