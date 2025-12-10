use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut points: Vec<(i32, i32)> = BufReader::new(File::open("src/input.txt").unwrap())
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|s| {
                s.split_once(',')
                    .map(|(a, b)| (a.trim().parse().unwrap(), b.trim().parse().unwrap()))
            })
        })
        .collect();
}

fn part_1(points: &mut Vec<(i32, i32)>) {}
