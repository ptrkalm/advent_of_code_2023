use std::collections::HashMap;

pub fn part_1(lines: Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| recover(line))
        .sum()
}

pub fn part_2(lines: Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| digitize(line))
        .map(|line| recover(&line))
        .sum()
}

fn recover(line: &String) -> usize {
    let filtered = line
        .chars()
        .filter(|&c| c.is_numeric())
        .collect::<Vec<char>>();

    format!("{}{}", filtered.first().unwrap(), filtered.last().unwrap()).parse::<usize>().unwrap()
}

fn digitize(line: &String) -> String {
    let mut result = line.clone();
    let from_letters = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e")
    ]);

    for (from, to) in from_letters {
        result = result.replace(from, to);
    }

    result
}