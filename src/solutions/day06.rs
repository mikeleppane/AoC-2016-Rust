use std::collections::HashMap;

use aoc_2016_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day06.txt";

pub struct Day06 {
    input: Vec<String>,
    charlist: Vec<HashMap<char, usize>>,
}

impl Day06 {
    pub fn new() -> Self {
        Self {
            input: vec![],
            charlist: vec![],
        }
    }
}

impl Runner for Day06 {
    fn name(&self) -> (usize, usize) {
        (2016, 6)
    }

    fn parse(&mut self) {
        self.input = read_lines(INPUT);
        for _ in 0..self.input[0].len() {
            self.charlist.push(HashMap::<char, usize>::new());
        }

        for entry in &self.input {
            for (i, c) in entry.chars().enumerate() {
                *self.charlist[i].entry(c).or_insert(0) += 1;
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut result = "".to_string();
        for cl in &self.charlist {
            let mut v: Vec<(&char, &usize)> = cl.iter().collect();
            v.sort_by(|a, b| b.1.cmp(a.1));
            result.push(*v[0].0);
        }
        output(result)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut result = "".to_string();
        for cl in &self.charlist {
            let mut v: Vec<(&char, &usize)> = cl.iter().collect();
            v.sort_by(|a, b| a.1.cmp(b.1));
            result.push(*v[0].0);
        }
        output(result)
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day06 = Day06::new();
        day06.parse();
        let output = day06.part1();
        assert_eq!(output[0], "liwvqppc")
    }

    #[test]
    fn part2_works() {
        let mut day06 = Day06::new();
        day06.parse();
        let output = day06.part2();
        assert_eq!(output[0], "caqfbzlh")
    }
}
