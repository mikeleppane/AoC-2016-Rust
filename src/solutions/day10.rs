use std::collections::HashMap;

use aoc_2016_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day10.txt";

#[derive(Default, Debug)]
pub struct Day10 {
    bot: HashMap<usize, Bot>,
    bins: HashMap<usize, Vec<usize>>,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day10 {
    fn name(&self) -> (usize, usize) {
        (2016, 10)
    }

    fn parse(&mut self) {
        for line in read_lines(INPUT) {
            let command = line.split_whitespace().collect::<Vec<&str>>();
            if command[2] == "goes" {
                let value = command[1].parse::<Chip>().unwrap();
                let bot_num = command[5].parse::<usize>().unwrap();
                let bot = self.bot.entry(bot_num).or_insert(Bot::new());
                bot.chips.push(value);
            } else if command[2] == "gives" {
                let bot_num = command[1].parse::<usize>().unwrap();
                let low = Recipient::parse(command[5], command[6]);
                let high = Recipient::parse(command[10], command[11]);
                let bot = self.bot.entry(bot_num).or_insert(Bot::new());
                bot.low = low;
                bot.high = high;
            }
        }
        println!("{:?}", self.bot[&125])
    }

    fn part1(&mut self) -> Vec<String> {
        let mut comparison_bot = 0;
        loop {
            let mut bot_num: Vec<usize> = self
                .bot
                .iter()
                .filter(|(_, bot)| bot.chips.len() == 2)
                .map(|(num, _)| *num)
                .collect();
            if let Some(num) = bot_num.pop() {
                let bot = self.bot.get_mut(&num).unwrap();
                let low = *bot.chips.iter().min().unwrap();
                let high = *bot.chips.iter().max().unwrap();

                let low_action = bot.low.clone();
                let high_action = bot.high.clone();
                bot.chips.clear();

                if low == 17 && high == 61 {
                    comparison_bot = num;
                }

                match low_action {
                    Recipient::Bot(num) => self.bot.get_mut(&num).unwrap().chips.push(low),
                    Recipient::Output(num) => self.bins.entry(num).or_default().push(low),
                    _ => panic!("Invalid recipient"),
                }

                match high_action {
                    Recipient::Bot(num) => {
                        let bot = self.bot.get_mut(&num).unwrap();
                        bot.chips.push(high);
                    }
                    Recipient::Output(num) => {
                        let bin = self.bins.entry(num).or_default();
                        bin.push(high);
                    }
                    _ => panic!("Invalid recipient"),
                }
            } else {
                break;
            }
        }
        output(comparison_bot)
    }

    fn part2(&mut self) -> Vec<String> {
        self.part1();
        let bin0 = self.bins.get(&0).unwrap()[0];
        let bin1 = self.bins.get(&1).unwrap()[0];
        let bin2 = self.bins.get(&2).unwrap()[0];

        output(bin0 * bin1 * bin2)
    }
}

// ---------------------------------------------------

type Chip = usize;

#[derive(Debug, Clone)]
enum Recipient {
    Empty,
    Bot(usize),
    Output(usize),
}

impl Recipient {
    fn parse(which: &str, num: &str) -> Self {
        match which {
            "bot" => Self::Bot(num.parse::<usize>().unwrap()),
            "output" => Self::Output(num.parse::<usize>().unwrap()),
            _ => panic!("Invalid recipient type {}", which),
        }
    }
}

#[derive(Debug, Clone)]
struct Bot {
    low: Recipient,
    high: Recipient,
    chips: Vec<Chip>,
}

impl Bot {
    fn new() -> Self {
        Self {
            low: Recipient::Empty,
            high: Recipient::Empty,
            chips: Vec::new(),
        }
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day = Day10::new();
        day.parse();
        let output = day.part1();
        assert_eq!(output[0], "86")
    }

    #[test]
    fn part2_works() {
        let mut day = Day10::new();
        day.parse();
        let output = day.part2();
        assert_eq!(output[0], "22847")
    }
}
