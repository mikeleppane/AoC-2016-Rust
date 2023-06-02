use aoc_2016_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day03.txt";

pub struct Day03 {
    list: Vec<(i32, i32, i32)>,
}

impl Day03 {
    pub fn new() -> Self {
        Self { list: vec![] }
    }
}

impl Runner for Day03 {
    fn name(&self) -> (usize, usize) {
        (2016, 3)
    }

    fn parse(&mut self) {
        for line in read_lines(INPUT) {
            if !line.trim().is_empty() {
                let mut iter = line.split_ascii_whitespace();
                let side1 = iter.next().unwrap().parse::<i32>().unwrap();
                let side2 = iter.next().unwrap().parse::<i32>().unwrap();
                let side3 = iter.next().unwrap().parse::<i32>().unwrap();
                self.list.push((side1, side2, side3));
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut possible = 0;
        for check in &self.list {
            possible += valid_triangle(check.0, check.1, check.2)
        }
        output(possible)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut possible = 0;
        for i in (0..self.list.len()).step_by(3) {
            possible += valid_triangle(self.list[i].0, self.list[i + 1].0, self.list[i + 2].0);
            possible += valid_triangle(self.list[i].1, self.list[i + 1].1, self.list[i + 2].1);
            possible += valid_triangle(self.list[i].2, self.list[i + 1].2, self.list[i + 2].2);
        }
        output(possible)
    }
}

fn valid_triangle(a: i32, b: i32, c: i32) -> i32 {
    if a + b > c && a + c > b && b + c > a {
        return 1;
    }
    0
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day01 = Day03::new();
        day01.parse();
        let output = day01.part1();
        assert_eq!(output[0], "869")
    }

    #[test]
    fn part2_works() {
        let mut day02 = Day03::new();
        day02.parse();
        let output = day02.part2();
        assert_eq!(output[0], "1544")
    }
}
