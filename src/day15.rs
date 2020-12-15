use std::collections::HashMap;

pub fn run_game(mut spoken: Vec<usize>, stop: usize) -> usize {
    let mut seen: HashMap<usize, usize> = HashMap::new();
    for i in 0..spoken.len() - 1 {
        seen.insert(spoken[i], i);
    }
    for turn in spoken.len()..stop {
        if let Some(i) = seen.get(&spoken[turn - 1]) {
            spoken.push(turn - i - 1);
        } else {
            spoken.push(0);
        }
        seen.insert(spoken[turn - 1], turn - 1);
    }
    spoken.pop().unwrap()
}

pub fn part1(inp: String) {
    println!(
        "{}",
        run_game(
            inp.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            2020
        )
    );
}

pub fn part2(inp: String) {
    println!(
        "{}",
        run_game(
            inp.split(',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>(),
            30000000
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_game() {
        assert!(run_game(vec![0, 3, 6], 4) == 0);
        assert!(run_game(vec![0, 3, 6], 5) == 3);
        assert!(run_game(vec![0, 3, 6], 6) == 3);
        assert!(run_game(vec![0, 3, 6], 7) == 1);
        assert!(run_game(vec![0, 3, 6], 8) == 0);
        assert!(run_game(vec![0, 3, 6], 9) == 4);
        assert!(run_game(vec![0, 3, 6], 10) == 0);
        assert!(run_game(vec![0, 3, 6], 2020) == 436);
        assert!(run_game(vec![1, 3, 2], 2020) == 1);
        assert!(run_game(vec![2, 1, 3], 2020) == 10);
        assert!(run_game(vec![1, 2, 3], 2020) == 27);
        assert!(run_game(vec![2, 3, 1], 2020) == 78);
        assert!(run_game(vec![3, 2, 1], 2020) == 438);
        assert!(run_game(vec![3, 1, 2], 2020) == 1836);
    }

    // #[test]
    // fn test_run_game_big_n() {
    //     assert!(run_game(vec![0, 3, 6], 30000000) == 175594);
    // }
}
