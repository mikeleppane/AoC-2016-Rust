use std::collections::HashMap;

use aoc_2016_rust::{output, Runner};

#[derive(Debug, Default)]
pub struct Day13 {
    dist: HashMap<(i32, i32), i32>,
}

impl Day13 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day13 {
    fn name(&self) -> (usize, usize) {
        (2016, 13)
    }

    fn parse(&mut self) {}

    fn part1(&mut self) -> Vec<String> {
        let maze = Maze::new(1358);
        let mut stack = vec![(0, 1), (1, 0), (2, 1), (1, 2)];
        self.dist.insert((1, 1), 0);

        while let Some(loc) = stack.pop() {
            if maze.is_wall_at(loc.0, loc.1) {
                continue;
            }

            let mut min = std::i32::MAX;
            min = min.min(*self.dist.entry((loc.0 + 1, loc.1)).or_insert(std::i32::MAX));
            min = min.min(*self.dist.entry((loc.0, loc.1 + 1)).or_insert(std::i32::MAX));
            min = min.min(*self.dist.entry((loc.0 - 1, loc.1)).or_insert(std::i32::MAX));
            min = min.min(*self.dist.entry((loc.0, loc.1 - 1)).or_insert(std::i32::MAX));
            min = min.saturating_add(1);

            let cur = self.dist.entry(loc).or_insert(std::i32::MAX);
            if min < *cur {
                *cur = min;

                if loc.0 < 50 {
                    stack.push((loc.0 + 1, loc.1));
                }
                if loc.1 < 50 {
                    stack.push((loc.0, loc.1 + 1));
                }

                if loc.0 > 0 {
                    stack.push((loc.0 - 1, loc.1));
                }
                if loc.1 > 0 {
                    stack.push((loc.0, loc.1 - 1));
                }
            }
        }
        output(self.dist.get(&(31, 39)).unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        let count = self.dist.values().filter(|&v| *v <= 50).count();
        output(count)
    }
}

// ---------------------------------------------------

struct Maze {
    number: i32,
}

impl Maze {
    fn new(number: i32) -> Self {
        Self { number }
    }

    fn is_wall_at(&self, x: i32, y: i32) -> bool {
        let n = x * x + 3 * x + 2 * x * y + y + y * y + self.number;
        let ones = n.count_ones();
        ones % 2 == 1
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day = Day13::new();
        let output = day.part1();
        assert_eq!(output[0], "96")
    }

    #[test]
    fn part2_works() {
        let mut day = Day13::new();
        day.part1();
        let output = day.part2();
        assert_eq!(output[0], "141")
    }
}
