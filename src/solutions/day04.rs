use std::collections::HashMap;

use aoc_2016_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day04.txt";

pub struct Day04 {
    room: Vec<Room>,
}

impl Day04 {
    pub fn new() -> Self {
        Self { room: vec![] }
    }
}

impl Runner for Day04 {
    fn name(&self) -> (usize, usize) {
        (2016, 4)
    }

    fn parse(&mut self) {
        for line in read_lines(INPUT) {
            self.room.push(Room::new(line.as_str()));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let total = self
            .room
            .iter()
            .filter_map(|r| {
                if r.is_real() {
                    return Some(r.sector);
                }
                None
            })
            .sum::<i32>();
        output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let total: i32 = self
            .room
            .iter()
            .filter_map(|r| {
                if r.is_real() && r.decrypt().contains("northpole") {
                    return Some(r.sector);
                }
                None
            })
            .next()
            .expect("Did not find the correct room (northpole)");
        output(total)
    }
}

#[derive(Debug)]
struct Room {
    name: String,
    sector: i32,
    checksum: String,
}

impl Room {
    fn new(room: &str) -> Self {
        let (room_str, checksum) = room.split_once('[').unwrap();
        let idx = room_str.rfind('-').unwrap();
        let (name, sector_str) = room_str.split_at(idx);
        let sector = sector_str[1..].parse::<i32>().unwrap();
        Self {
            name: name.to_string(),
            sector,
            checksum: checksum[..checksum.len() - 1].to_string(),
        }
    }

    fn is_real(&self) -> bool {
        let mut hm = HashMap::new();
        for c in self.name.chars().filter(|x| x.is_alphabetic()) {
            *hm.entry(c).or_insert(0) += 1;
        }
        let mut list = hm.iter().collect::<Vec<(&char, &i32)>>();
        list.sort_by(|a, b| {
            if b.1 == a.1 {
                a.0.cmp(b.0)
            } else {
                b.1.cmp(a.1)
            }
        });

        self.checksum == list.iter().take(5).map(|(&k, _)| k).collect::<String>()
    }

    fn decrypt(&self) -> String {
        let mut result = String::from("");
        let m = (self.sector % 26) as u8;
        for c in self.name.chars() {
            if c == '-' {
                result.push(' ');
            } else {
                result.push((b'a' + (c as u8 - b'a' + m) % 26) as char);
            }
        }

        result
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day04 = Day04::new();
        day04.parse();
        let output = day04.part1();
        assert_eq!(output[0], "245102")
    }

    #[test]
    fn part2_works() {
        let mut day04 = Day04::new();
        day04.parse();
        let output = day04.part2();
        assert_eq!(output[0], "324")
    }
}
