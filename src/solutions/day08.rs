use std::collections::HashSet;

use aoc_2016_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day08.txt";

#[derive(Default)]
pub struct Day08 {
    inst: Vec<Instruction>,
    answer: HashSet<(usize, usize)>,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day08 {
    fn name(&self) -> (usize, usize) {
        (2016, 8)
    }

    fn parse(&mut self) {
        for line in read_lines(INPUT) {
            self.inst.push(Instruction::new(&line));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        for i in &self.inst {
            match i {
                Instruction::Rect(a, b) => {
                    for x in 0..*b {
                        for y in 0..*a {
                            self.answer.insert((x, y));
                        }
                    }
                }
                Instruction::Row(idx, amt) => {
                    let mut new = HashSet::new();
                    for (r, c) in self.answer.iter() {
                        if *r == *idx {
                            let new_c = (c + amt) % 50;
                            new.insert((*r, new_c));
                        } else {
                            new.insert((*r, *c));
                        }
                    }
                    self.answer = new;
                }
                Instruction::Col(col, amt) => {
                    let mut new = HashSet::new();
                    for (r, c) in self.answer.iter() {
                        if *c == *col {
                            let new_r = (r + amt) % 6;
                            new.insert((new_r, *c));
                        } else {
                            new.insert((*r, *c));
                        }
                    }
                    self.answer = new;
                }
            }
        }
        output(self.answer.len())
    }

    fn part2(&mut self) -> Vec<String> {
        self.part1();
        let mut answer = vec![];
        for r in 0..6 {
            let mut line = "".to_string();
            for c in 0..50 {
                if self.answer.contains(&(r, c)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            answer.push(line)
        }
        for line in &answer {
            println!("{}", line);
        }
        answer
    }
}

// ---------------------------------------------------

enum Instruction {
    Rect(usize, usize),
    Row(usize, usize),
    Col(usize, usize),
}

impl Instruction {
    fn new(s: &str) -> Self {
        let (cmd, rest) = s.split_once(' ').unwrap();
        match cmd {
            "rect" => {
                let (a, b) = rest.split_once('x').unwrap();
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();
                Self::Rect(a, b)
            }
            "rotate" => {
                let (cmd, rest) = rest.split_once('=').unwrap();
                let (a, b) = rest.split_once(" by ").unwrap();
                let (a, b) = (a.parse().unwrap(), b.parse().unwrap());
                if cmd.starts_with("row") {
                    Self::Row(a, b)
                } else {
                    Self::Col(a, b)
                }
            }
            _ => panic!("Unknown command"),
        }
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day08 = Day08::new();
        day08.parse();
        let output = day08.part1();
        assert_eq!(output[0], "110")
    }

    #[test]
    fn part2_works() {
        let mut day08 = Day08::new();
        day08.parse();
        let _ = day08.part2();
    }
}
