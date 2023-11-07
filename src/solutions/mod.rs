use std::collections::HashMap;

use aoc_2016_rust::{run_solution, Runner};
use day01::Day01;
use day02::Day02;

use crate::solutions::day03::Day03;
use crate::solutions::day04::Day04;
use crate::solutions::day05::Day05;
use crate::solutions::day06::Day06;
use crate::solutions::day07::Day07;
use crate::solutions::day08::Day08;
use crate::solutions::day09::Day09;
use crate::solutions::day10::Day10;
use crate::solutions::day12::Day12;
use crate::solutions::day13::Day13;
use crate::Selector;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day12;
mod day13;

pub fn run(which: Selector) {
    let mut day01 = Day01::new();
    let mut day02 = Day02::new();
    let mut day03 = Day03::new();
    let mut day04 = Day04::new();
    let mut day05 = Day05::new();
    let mut day06 = Day06::new();
    let mut day07 = Day07::new();
    let mut day08 = Day08::new();
    let mut day09 = Day09::new();
    let mut day10 = Day10::new();
    let mut day12 = Day12::new();
    let mut day13 = Day13::new();

    let mut days: HashMap<u8, &mut dyn Runner> = HashMap::new();
    days.insert(1, &mut day01);
    days.insert(2, &mut day02);
    days.insert(3, &mut day03);
    days.insert(4, &mut day04);
    days.insert(5, &mut day05);
    days.insert(6, &mut day06);
    days.insert(7, &mut day07);
    days.insert(8, &mut day08);
    days.insert(9, &mut day09);
    days.insert(10, &mut day10);
    days.insert(12, &mut day12);
    days.insert(13, &mut day13);

    match which {
        Selector::Last => {
            let last = *days.keys().max().unwrap();
            if let Some(d) = days.get_mut(&last) {
                run_solution(*d);
            }
        }
        Selector::All => {
            for d in days.values_mut() {
                run_solution(*d);
            }
        }
        Selector::One(num) => {
            if let Some(d) = days.get_mut(&(num as u8)) {
                run_solution(*d);
            }
        }
    }
}
