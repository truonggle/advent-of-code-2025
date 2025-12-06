use std::{fs::read_to_string, io::Result};

fn main() -> Result<()> {
    let s = read_to_string("src/input.txt").unwrap();
    part_1(&s);
    Ok(())
}

fn part_1(s: &str) {
    let mut lines = s
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect::<Vec<_>>();
    let ops = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|x| x.chars().next().unwrap())
        .collect::<Vec<_>>();
    let rows = lines
        .into_iter()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let total: u32 = (0..ops.len())
        .map(|c| {
            let op = ops[c];
            rows.iter()
                .map(|r| r[c])
                .reduce(|a, b| if op == '+' { a + b } else { a * b })
                .unwrap()
        })
        .sum();
    print!("{total}");
}
