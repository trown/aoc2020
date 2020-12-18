use std::cmp::min;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum FloorPos {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for FloorPos {
    fn from(c: char) -> Self {
        match c {
            '.' => FloorPos::Floor,
            'L' => FloorPos::Empty,
            '#' => FloorPos::Occupied,
            _ => panic!(),
        }
    }
}

impl FloorPos {
    pub fn is_occupied(&self) -> bool {
        self == &FloorPos::Occupied
    }
    pub fn is_floor(&self) -> bool {
        self == &FloorPos::Floor
    }
}

#[derive(Debug)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
struct Layout {
    state: Vec<Vec<FloorPos>>,
    seen: HashSet<Vec<Vec<FloorPos>>>,
    part: Part,
}

impl Layout {
    fn new(state: Vec<Vec<FloorPos>>, part: Part) -> Self {
        let mut seen = HashSet::new();
        seen.insert(state.clone());
        Layout { state, seen, part }
    }
    fn tick(&mut self) -> Option<&mut Self> {
        let mut swap = vec![vec![FloorPos::Floor; self.state[0].len()]; self.state.len()];

        for (i, row) in swap.iter_mut().enumerate().take(self.state.len()) {
            for (j, pos) in row.iter_mut().enumerate().take(self.state[0].len()) {
                let fp = self.state[i][j];
                let occupied_neighbors = match &self.part {
                    Part::Part1 => self.occupied_neighbor_count(i, j),
                    Part::Part2 => self.occupied_neighbor_count_part2(i, j),
                };

                let next_pos = match (fp, occupied_neighbors, &self.part) {
                    (FloorPos::Empty, 0, _) => FloorPos::Occupied,
                    (FloorPos::Occupied, x, Part::Part1) if x >= 4 => FloorPos::Empty,
                    (FloorPos::Occupied, x, Part::Part2) if x >= 5 => FloorPos::Empty,
                    (otherwise, _, _) => otherwise,
                };

                *pos = next_pos;
            }
        }
        self.state = swap;
        if self.seen.insert(self.state.clone()) {
            None
        } else {
            Some(self)
        }
    }

    fn occupied_neighbor_count(&self, i: usize, j: usize) -> usize {
        let (mut north, mut south, mut east, mut west, mut ne, mut nw, mut se, mut sw) =
            (0, 0, 0, 0, 0, 0, 0, 0);

        if i > 0 {
            north = self.state[i - 1][j].is_occupied() as usize;
        }
        if i < self.state.len() - 1 {
            south = self.state[i + 1][j].is_occupied() as usize;
        }
        if j < self.state[0].len() - 1 {
            east = self.state[i][j + 1].is_occupied() as usize;
        }
        if j > 0 {
            west = self.state[i][j - 1].is_occupied() as usize;
        }
        if i > 0 && j < self.state[0].len() - 1 {
            ne = self.state[i - 1][j + 1].is_occupied() as usize;
        }
        if i > 0 && j > 0 {
            nw = self.state[i - 1][j - 1].is_occupied() as usize;
        }
        if i < self.state.len() - 1 && j < self.state[0].len() - 1 {
            se = self.state[i + 1][j + 1].is_occupied() as usize;
        }
        if i < self.state.len() - 1 && j > 0 {
            sw = self.state[i + 1][j - 1].is_occupied() as usize;
        }

        north + south + east + west + ne + nw + se + sw
    }

    fn occupied_neighbor_count_part2(&self, i: usize, j: usize) -> usize {
        let (mut north, mut south, mut east, mut west, mut ne, mut nw, mut se, mut sw) =
            (0, 0, 0, 0, 0, 0, 0, 0);

        if i > 0 {
            if let Some(o) = (1..i + 1)
                .map(|x| self.state[i - x][j])
                .find(|tile| !tile.is_floor())
            {
                north = o.is_occupied() as usize;
            }
        }
        if i < self.state.len() - 1 {
            if let Some(o) = (1..self.state.len() - i)
                .map(|x| self.state[i + x][j])
                .find(|tile| !tile.is_floor())
            {
                west = o.is_occupied() as usize;
            }
        }
        if j < self.state[0].len() - 1 {
            if let Some(o) = (1..self.state[0].len() - j)
                .map(|x| self.state[i][j + x])
                .find(|tile| !tile.is_floor())
            {
                east = o.is_occupied() as usize;
            }
        }
        if j > 0 {
            if let Some(o) = (1..j + 1)
                .map(|x| self.state[i][j - x])
                .find(|tile| !tile.is_floor())
            {
                south = o.is_occupied() as usize;
            }
        }
        if i > 0 && j < self.state[0].len() - 1 {
            if let Some(o) = (1..min(i + 1, self.state[0].len() - j))
                .map(|x| self.state[i - x][j + x])
                .find(|tile| !tile.is_floor())
            {
                ne = o.is_occupied() as usize;
            }
        }
        if i > 0 && j > 0 {
            if let Some(o) = (1..min(i + 1, j + 1))
                .map(|x| self.state[i - x][j - x])
                .find(|tile| !tile.is_floor())
            {
                nw = o.is_occupied() as usize;
            }
        }
        if i < self.state.len() - 1 && j < self.state[0].len() - 1 {
            if let Some(o) = (1..min(self.state.len() - i, self.state[0].len() - j))
                .map(|x| self.state[i + x][j + x])
                .find(|tile| !tile.is_floor())
            {
                se = o.is_occupied() as usize;
            }
        }
        if i < self.state.len() - 1 && j > 0 {
            if let Some(o) = (1..min(self.state.len() - i, j + 1))
                .map(|x| self.state[i + x][j - x])
                .find(|tile| !tile.is_floor())
            {
                sw = o.is_occupied() as usize;
            }
        }

        north + south + east + west + ne + nw + se + sw
    }
}

pub fn part1(inp: String) {
    let mut layout = Layout::new(
        inp.split('\n')
            .map(|row| row.chars().map(FloorPos::from).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        Part::Part1,
    );
    loop {
        if let Some(stale) = layout.tick() {
            println!(
                "{}",
                stale
                    .state
                    .iter()
                    .flatten()
                    .filter(|pos| pos.is_occupied())
                    .count()
            );
            return;
        }
    }
}

pub fn part2(inp: String) {
    let mut layout = Layout::new(
        inp.split('\n')
            .map(|row| row.chars().map(FloorPos::from).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        Part::Part2,
    );

    loop {
        if let Some(stale) = layout.tick() {
            println!(
                "{}",
                stale
                    .state
                    .iter()
                    .flatten()
                    .filter(|pos| pos.is_occupied())
                    .count()
            );
            return;
        }
    }
}
