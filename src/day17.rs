use std::collections::HashMap;
use std::collections::HashSet;

type Cell = (i32, i32, i32);
type Colony = HashSet<Cell>;
type Cell2 = (i32, i32, i32, i32);
type Colony2 = HashSet<Cell2>;

fn neighbours(&(x, y, z): &Cell) -> Vec<Cell> {
    vec![
        (x - 1, y - 1, z - 1),
        (x, y - 1, z - 1),
        (x + 1, y - 1, z - 1),
        (x - 1, y, z - 1),
        (x, y, z - 1),
        (x + 1, y, z - 1),
        (x - 1, y + 1, z - 1),
        (x, y + 1, z - 1),
        (x + 1, y + 1, z - 1),
        (x - 1, y - 1, z),
        (x, y - 1, z),
        (x + 1, y - 1, z),
        (x - 1, y, z),
        (x + 1, y, z),
        (x - 1, y + 1, z),
        (x, y + 1, z),
        (x + 1, y + 1, z),
        (x - 1, y - 1, z + 1),
        (x, y - 1, z + 1),
        (x + 1, y - 1, z + 1),
        (x - 1, y, z + 1),
        (x, y, z + 1),
        (x + 1, y, z + 1),
        (x - 1, y + 1, z + 1),
        (x, y + 1, z + 1),
        (x + 1, y + 1, z + 1),
    ]
}

fn neighbours2(&(x, y, z, w): &Cell2) -> Vec<Cell2> {
    vec![
        (x - 1, y - 1, z - 1, w - 1),
        (x, y - 1, z - 1, w - 1),
        (x + 1, y - 1, z - 1, w - 1),
        (x - 1, y, z - 1, w - 1),
        (x, y, z - 1, w - 1),
        (x + 1, y, z - 1, w - 1),
        (x - 1, y + 1, z - 1, w - 1),
        (x, y + 1, z - 1, w - 1),
        (x + 1, y + 1, z - 1, w - 1),
        (x - 1, y - 1, z, w - 1),
        (x, y - 1, z, w - 1),
        (x + 1, y - 1, z, w - 1),
        (x - 1, y, z, w - 1),
        (x, y, z, w - 1),
        (x + 1, y, z, w - 1),
        (x - 1, y + 1, z, w - 1),
        (x, y + 1, z, w - 1),
        (x + 1, y + 1, z, w - 1),
        (x - 1, y - 1, z + 1, w - 1),
        (x, y - 1, z + 1, w - 1),
        (x + 1, y - 1, z + 1, w - 1),
        (x - 1, y, z + 1, w - 1),
        (x, y, z + 1, w - 1),
        (x + 1, y, z + 1, w - 1),
        (x - 1, y + 1, z + 1, w - 1),
        (x, y + 1, z + 1, w - 1),
        (x + 1, y + 1, z + 1, w - 1),
        (x - 1, y - 1, z - 1, w),
        (x, y - 1, z - 1, w),
        (x + 1, y - 1, z - 1, w),
        (x - 1, y, z - 1, w),
        (x, y, z - 1, w),
        (x + 1, y, z - 1, w),
        (x - 1, y + 1, z - 1, w),
        (x, y + 1, z - 1, w),
        (x + 1, y + 1, z - 1, w),
        (x - 1, y - 1, z, w),
        (x, y - 1, z, w),
        (x + 1, y - 1, z, w),
        (x - 1, y, z, w),
        (x + 1, y, z, w),
        (x - 1, y + 1, z, w),
        (x, y + 1, z, w),
        (x + 1, y + 1, z, w),
        (x - 1, y - 1, z + 1, w),
        (x, y - 1, z + 1, w),
        (x + 1, y - 1, z + 1, w),
        (x - 1, y, z + 1, w),
        (x, y, z + 1, w),
        (x + 1, y, z + 1, w),
        (x - 1, y + 1, z + 1, w),
        (x, y + 1, z + 1, w),
        (x + 1, y + 1, z + 1, w),
        (x - 1, y - 1, z - 1, w + 1),
        (x, y - 1, z - 1, w + 1),
        (x + 1, y - 1, z - 1, w + 1),
        (x - 1, y, z - 1, w + 1),
        (x, y, z - 1, w + 1),
        (x + 1, y, z - 1, w + 1),
        (x - 1, y + 1, z - 1, w + 1),
        (x, y + 1, z - 1, w + 1),
        (x + 1, y + 1, z - 1, w + 1),
        (x - 1, y - 1, z, w + 1),
        (x, y - 1, z, w + 1),
        (x + 1, y - 1, z, w + 1),
        (x - 1, y, z, w + 1),
        (x, y, z, w + 1),
        (x + 1, y, z, w + 1),
        (x - 1, y + 1, z, w + 1),
        (x, y + 1, z, w + 1),
        (x + 1, y + 1, z, w + 1),
        (x - 1, y - 1, z + 1, w + 1),
        (x, y - 1, z + 1, w + 1),
        (x + 1, y - 1, z + 1, w + 1),
        (x - 1, y, z + 1, w + 1),
        (x, y, z + 1, w + 1),
        (x + 1, y, z + 1, w + 1),
        (x - 1, y + 1, z + 1, w + 1),
        (x, y + 1, z + 1, w + 1),
        (x + 1, y + 1, z + 1, w + 1),
    ]
}

fn neighbour_counts(col: &Colony) -> HashMap<Cell, i32> {
    let mut ncnts = HashMap::new();
    for cell in col.iter().flat_map(neighbours) {
        *ncnts.entry(cell).or_insert(0) += 1;
    }
    ncnts
}

fn neighbour_counts2(col: &Colony2) -> HashMap<Cell2, i32> {
    let mut ncnts = HashMap::new();
    for cell in col.iter().flat_map(neighbours2) {
        *ncnts.entry(cell).or_insert(0) += 1;
    }
    ncnts
}

fn generation(col: Colony) -> Colony {
    neighbour_counts(&col)
        .into_iter()
        .filter_map(|(cell, cnt)| match (cnt, col.contains(&cell)) {
            (2, true) | (3, ..) => Some(cell),
            _ => None,
        })
        .collect()
}

fn generation2(col: Colony2) -> Colony2 {
    neighbour_counts2(&col)
        .into_iter()
        .filter_map(|(cell, cnt)| match (cnt, col.contains(&cell)) {
            (2, true) | (3, ..) => Some(cell),
            _ => None,
        })
        .collect()
}

fn life(init: HashSet<Cell>, iters: i32) -> usize {
    let mut col: Colony = init.into_iter().collect();
    for _i in 0..iters {
        col = generation(col);
    }
    col.len()
}

fn life2(init: HashSet<Cell2>, iters: i32) -> usize {
    let mut col: Colony2 = init.into_iter().collect();
    for _i in 0..iters {
        col = generation2(col);
    }
    col.len()
}

pub fn part1(inp: String) {
    let mut init = HashSet::new();
    for (i, line) in inp.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                init.insert((j as i32, i as i32, 0));
            }
        }
    }
    println!("{}", life(init, 6));
}

pub fn part2(inp: String) {
    let mut init = HashSet::new();
    for (i, line) in inp.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                init.insert((j as i32, i as i32, 0, 0));
            }
        }
    }
    println!("{}", life2(init, 6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_life() {
        let init = vec![(1, 0, 0), (2, 1, 0), (0, 2, 0), (1, 2, 0), (2, 2, 0)]
            .into_iter()
            .collect();
        assert_eq!(life(init, 6), 112);
    }

    #[test]
    fn test_life2() {
        let init = vec![
            (1, 0, 0, 0),
            (2, 1, 0, 0),
            (0, 2, 0, 0),
            (1, 2, 0, 0),
            (2, 2, 0, 0),
        ]
        .into_iter()
        .collect();
        assert_eq!(life2(init, 6), 848);
    }
}
