use std::iter::Sum;
use std::ops::Add;

pub fn part_1(lines: Vec<String>) -> usize {
    parse(lines)
        .iter()
        .enumerate()
        .filter(|(_, line)| line.iter().all(|point| is_possible(point)))
        .map(|(n, _)| n + 1)
        .sum()
}

pub fn part_2(lines: Vec<String>) -> usize {
    parse(lines)
        .iter()
        .map(|line| vec!(
            line.iter().max_by_key(|point| point.0).unwrap().0,
            line.iter().max_by_key(|point| point.1).unwrap().1,
            line.iter().max_by_key(|point| point.2).unwrap().2
        ).iter().product::<usize>())
        .sum()
}

fn is_possible(point: &Point) -> bool {
    point.0 <= 12 && point.1 <= 13 && point.2 <= 14
}

fn parse(lines: Vec<String>) -> Vec<Vec<Point>> {
    let result = lines
        .iter()
        .map(|line| line.split_once(":").unwrap().1)
        .map(|line| line
            .split(";")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&subset| subset
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|cube| match cube.trim().split_once(" ").unwrap() {
                    (n, "red") => Point(n.parse::<usize>().unwrap(), 0, 0),
                    (n, "green") => Point(0, n.parse::<usize>().unwrap(), 0),
                    (n, "blue") => Point(0, 0, n.parse::<usize>().unwrap()),
                    _ => unreachable!()
                })
                .collect::<Vec<Point>>())
            .collect::<Vec<Vec<Point>>>()
        )
        .collect::<Vec<Vec<Vec<Point>>>>();


    result
        .iter()
        .map(|line| line
            .into_iter()
            .map(|subset| subset
                .iter()
                .sum())
            .collect::<Vec<Point>>())
        .collect::<Vec<Vec<Point>>>()
}

#[derive(Debug, Copy, Clone)]
struct Point(usize, usize, usize);

impl<'a> Sum<&'a Point> for Point {
    fn sum<I: Iterator<Item=&'a Point>>(iter: I) -> Self {
        let mut result = Point(0, 0, 0);
        for point in iter {
            result = result + *point;
        }

        result
    }
}

impl Sum<Point> for Point {
    fn sum<I: Iterator<Item=Point>>(iter: I) -> Self {
        let mut result = Point(0, 0, 0);
        for point in iter {
            result = result + point;
        }

        result
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}