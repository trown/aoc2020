#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Forward,
    North,
    South,
    East,
    West,
    Right,
    Left,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Instruction {
    direction: Direction,
    magnitude: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let direction = match &s[0..1] {
            "F" => Direction::Forward,
            "N" => Direction::North,
            "S" => Direction::South,
            "E" => Direction::East,
            "W" => Direction::West,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!(),
        };
        let magnitude = &s[1..].parse::<usize>().unwrap();
        Instruction {
            direction,
            magnitude: *magnitude,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    north: usize,
    south: usize,
    east: usize,
    west: usize,
    facing: Direction,
}

impl Position {
    pub fn next(self, i: Instruction) -> Self {
        let mut temp = self;
        match (i.direction, i.magnitude) {
            (Direction::Forward, x) => match self.facing {
                Direction::North => temp.north += x,
                Direction::South => temp.south += x,
                Direction::East => temp.east += x,
                Direction::West => temp.west += x,
                _ => panic!(),
            },
            (Direction::North, x) => temp.north += x,
            (Direction::South, x) => temp.south += x,
            (Direction::East, x) => temp.east += x,
            (Direction::West, x) => temp.west += x,
            (Direction::Right, 90) | (Direction::Left, 270) => match self.facing {
                Direction::North => temp.facing = Direction::East,
                Direction::South => temp.facing = Direction::West,
                Direction::East => temp.facing = Direction::South,
                Direction::West => temp.facing = Direction::North,
                _ => panic!(),
            },
            (Direction::Left, 90) | (Direction::Right, 270) => match self.facing {
                Direction::North => temp.facing = Direction::West,
                Direction::South => temp.facing = Direction::East,
                Direction::East => temp.facing = Direction::North,
                Direction::West => temp.facing = Direction::South,
                _ => panic!(),
            },
            (Direction::Left, 180) | (Direction::Right, 180) => match self.facing {
                Direction::North => temp.facing = Direction::South,
                Direction::South => temp.facing = Direction::North,
                Direction::East => temp.facing = Direction::West,
                Direction::West => temp.facing = Direction::East,
                _ => panic!(),
            },
            (_, _) => (),
        }
        temp
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position2 {
    north: usize,
    south: usize,
    east: usize,
    west: usize,
    waypoint: Position,
}

impl Position2 {
    pub fn next(self, i: Instruction) -> Self {
        let mut temp = self;
        match (i.direction, i.magnitude) {
            (Direction::Forward, x) => {
                temp.north += self.waypoint.north * x;
                temp.south += self.waypoint.south * x;
                temp.east += self.waypoint.east * x;
                temp.west += self.waypoint.west * x;
            }
            (Direction::North, x) => temp.waypoint.north += x,
            (Direction::South, x) => temp.waypoint.south += x,
            (Direction::East, x) => temp.waypoint.east += x,
            (Direction::West, x) => temp.waypoint.west += x,
            (Direction::Right, 90) | (Direction::Left, 270) => {
                temp.waypoint.north = self.waypoint.west;
                temp.waypoint.south = self.waypoint.east;
                temp.waypoint.east = self.waypoint.north;
                temp.waypoint.west = self.waypoint.south;
            }
            (Direction::Left, 90) | (Direction::Right, 270) => {
                temp.waypoint.north = self.waypoint.east;
                temp.waypoint.south = self.waypoint.west;
                temp.waypoint.east = self.waypoint.south;
                temp.waypoint.west = self.waypoint.north;
            }
            (Direction::Left, 180) | (Direction::Right, 180) => {
                temp.waypoint.north = self.waypoint.south;
                temp.waypoint.south = self.waypoint.north;
                temp.waypoint.east = self.waypoint.west;
                temp.waypoint.west = self.waypoint.east;
            }
            (_, _) => (),
        }
        temp
    }
}

pub fn part1(inp: String) {
    let init = Position {
        north: 0,
        south: 0,
        east: 0,
        west: 0,
        facing: Direction::East,
    };
    let end = inp
        .split("\n")
        .fold(init, |acc, ins| acc.next(Instruction::from(ins)));
    println!(
        "{:?}",
        (end.north as i32 - end.south as i32).abs() + (end.east as i32 - end.west as i32).abs()
    );
}

pub fn part2(inp: String) {
    let init = Position2 {
        north: 0,
        south: 0,
        east: 0,
        west: 0,
        waypoint: Position {
            north: 1,
            south: 0,
            east: 10,
            west: 0,
            facing: Direction::East,
        },
    };
    //let inp = "F10\nN3\nF7\nR90\nF11";
    let end = inp.split("\n").fold(init, |acc, ins| {
        println!("{:?}", acc);
        acc.next(Instruction::from(ins))
    });
    println!(
        "{:?},{:?}",
        &end,
        (end.north as i32 - end.south as i32).abs() + (end.east as i32 - end.west as i32).abs()
    );
}
