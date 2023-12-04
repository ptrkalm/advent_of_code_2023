mod solutions;
use solutions::*;
use std::fs::read_to_string;

fn main() {
    // println!("{:?}", day_1::part_1(read_lines(1)));
    // println!("{:?}", day_1::part_2(read_lines(1)));

    println!("{:?}", day_2::part_1(read_lines(2)));
    println!("{:?}", day_2::part_2(read_lines(2)));
}
fn read_lines(day: usize) -> Vec<String> {
    read_to_string(format!("inputs/day{}.txt", day))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
