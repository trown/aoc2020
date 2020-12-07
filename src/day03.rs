pub fn get_trees(tree_map: &str, rise: usize, run: usize) -> usize {
    tree_map
        .lines()
        .enumerate()
        .filter(|(i, _)| i % rise == 0)
        .filter(|(i, line)| line.chars().nth((run * i / rise) % line.len()) == Some('#'))
        .count()
}

pub fn part1(inp: String) {
    println!("{}", get_trees(&inp, 1, 3));
}

pub fn part2(inp: String) {
    println!(
        "{}",
        [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
            .iter()
            .map(|(rise, run)| get_trees(&inp, *rise, *run))
            .product::<usize>()
    );
}
