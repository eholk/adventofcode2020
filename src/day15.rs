use std::collections::HashMap;

const INPUT: &[usize] = &[6, 13, 1, 15, 2, 0];

pub fn run() {
    let last_spoken = play_game(INPUT, 2020);
    println!("Part 1: {}", last_spoken);
}

fn play_game(input: &[usize], turns: usize) -> usize {
    let mut last_turns = HashMap::new();
    let mut last_spoken = 0;
    for turn in 1..turns {
        last_spoken = if turn - 1 < input.len() {
            input[turn - 1]
        } else {
            match last_turns.insert(last_spoken, turn - 1) {
                Some(last_turn) => turn - last_turn,
                None => 0,
            }
        }
    }
    last_spoken
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
