use std::{fs::read_to_string, io::Result};

fn main() -> Result<()> {
    let s = read_to_string("src/input.txt").unwrap();
    //part_1(&s);
    part_2(&s);
    Ok(())
}

fn part_2(s: &str) {
    let lines: Vec<&str> = s.trim().lines().collect();
    if lines.is_empty() {
        return;
    }
    let width: usize = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let cols: Vec<usize> = (0..width).collect();

    let has_data = |c: &usize| {
        lines.iter().any(|line| {
            line.chars()
                .nth(*c)
                .is_some_and(|char| char.is_ascii_digit())
        })
    };
    let total: u64 = cols
        .chunk_by(|a, b| has_data(a) == has_data(b))
        .filter(|chunk| has_data(&chunk[0]))
        .map(|chunk| {
            let op = lines.last().unwrap().chars().nth(chunk[0]).unwrap_or(' ');
            chunk
                .iter()
                .map(|&c| {
                    lines
                        .iter()
                        .filter_map(|line| line.chars().nth(c).and_then(|char| char.to_digit(10)))
                        .fold(0u64, |acc, digit| acc * 10 + digit as u64)
                })
                .reduce(|acc, val| if op == '+' { acc + val } else { acc * val })
                .unwrap_or(0)
        })
        .sum();
    print!("{total}");
}

fn part_1(s: &str) {
    let lines: Vec<&str> = s.trim().lines().collect();
    let (op_line, number_lines) = lines.split_last().unwrap();
    let m: Vec<Vec<u64>> = number_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();
    let total: u64 = op_line
        .split_whitespace()
        .enumerate()
        .map(|(c, op)| {
            let col = m.iter().map(|row| row[c]);
            match op {
                "+" => col.sum::<u64>(),
                "*" => col.product::<u64>(),
                _ => 0,
            }
        })
        .sum();
    print!("{total}");
}
