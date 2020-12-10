pub fn find_chain(adapters: Vec<usize>) -> usize {
    let cnt = adapters
        .iter()
        .enumerate()
        .fold((0, 0), |(mut o, mut t), (i, a)| {
            if i < adapters.len() - 1 {
                match adapters[i + 1] - a {
                    1 => o += 1,
                    3 => t += 1,
                    _ => (),
                }
            }
            (o, t)
        });
    cnt.0 * (cnt.1 + 1)
}

pub fn find_all_chains(adapters: Vec<usize>) -> u64 {
    let cnt = adapters
        .iter()
        .enumerate()
        .fold((1, 1), |(mut streak, mut total), (i, a)| {
            if i < adapters.len() - 1 {
                match adapters[i + 1] - a {
                    1 => streak += 1,
                    3 => {
                        total = map_streak(streak) * total;
                        streak = 1;
                    }
                    _ => (),
                }
            } else {
                total = map_streak(streak) * total;
            }
            (streak, total)
        });
    cnt.1
}

pub fn map_streak(streak: u64) -> u64 {
    match streak {
        1 => 1,
        2 => 1,
        3 => 2,
        n => 2_u64.pow(n as u32 - 2) - (n - 4),
    }
}

pub fn part1(inp: String) {
    let mut adapters: Vec<usize> = inp.split("\n").map(|i| i.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort();
    println!("{:?}", find_chain(adapters));
}

pub fn part2(inp: String) {
    let mut adapters: Vec<usize> = inp.split("\n").map(|i| i.parse().unwrap()).collect();
    adapters.push(0);
    adapters.sort();
    println!("{:?}", find_all_chains(adapters));
}
