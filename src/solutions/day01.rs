use std::collections::HashSet;

use aoc_2016_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day01.txt";

pub struct Day01 {
    instr: Vec<Instruction>,
}

impl Day01 {
    pub fn new() -> Self {
        Self { instr: vec![] }
    }
}

impl Runner for Day01 {
    fn name(&self) -> (usize, usize) {
        (2016, 1)
    }

    fn parse(&mut self) {
        if let Some(line) = read_lines(INPUT).first() {
            for inst in line.split(", ") {
                let (dir, dist) = inst.split_at(1);
                let dist = dist.parse::<i32>().unwrap();
                match dir {
                    "R" => self.instr.push(Instruction::Right(dist)),
                    "L" => self.instr.push(Instruction::Left(dist)),
                    _ => panic!("corrupted input file!"),
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut x = 0;
        let mut y = 0;
        let mut dir = Direction::North;
        for i in &self.instr {
            let (new_dir, dist) = i.aim(dir);
            dir = new_dir;
            match dir {
                Direction::North => y -= dist,
                Direction::South => y += dist,
                Direction::East => x += dist,
                Direction::West => x -= dist,
            }
        }

        output(x.abs() + y.abs())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut visited = HashSet::<(i32, i32)>::new();
        visited.insert((0, 0));
        let mut x = 0;
        let mut y = 0;
        let mut dir = Direction::North;
        'outer: for i in &self.instr {
            let (new_dir, dist) = i.aim(dir);
            dir = new_dir;

            let (dx, dy) = match dir {
                Direction::North => (0, -1),
                Direction::South => (0, 1),
                Direction::East => (1, 0),
                Direction::West => (-1, 0),
            };

            for _ in 0..dist {
                x += dx;
                y += dy;
                if visited.contains(&(x, y)) {
                    break 'outer;
                }
                visited.insert((x, y));
            }
        }

        output(x.abs() + y.abs())
    }
}

// ---------------------------------------------------
enum Instruction {
    Left(i32),
    Right(i32),
}

impl Instruction {
    fn aim(&self, cur_dir: Direction) -> (Direction, i32) {
        match self {
            Self::Left(n) => (cur_dir.turn_left(), *n),
            Self::Right(n) => (cur_dir.turn_right(), *n),
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::North => Direction::East,
            Self::South => Direction::West,
            Self::East => Direction::South,
            Self::West => Direction::North,
        }
    }
    fn turn_left(&self) -> Self {
        match self {
            Self::North => Direction::West,
            Self::South => Direction::East,
            Self::East => Direction::North,
            Self::West => Direction::South,
        }
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2016_rust::Runner;

    use crate::solutions::day01::Day01;

    #[test]
    fn part1_works() {
        let mut day01 = Day01::new();
        day01.parse();
        let output = day01.part1();
        assert_eq!(output[0], "230")
    }

    #[test]
    fn part2_works() {
        let mut day01 = Day01::new();
        day01.parse();
        day01.part1();
        let output = day01.part2();
        assert_eq!(output[0], "154")
    }
}
