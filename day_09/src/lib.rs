use std::fs::{self};

pub fn read_input(path: &str) -> String {
    fs::read_to_string(path).expect("File not found")
}

pub fn parse_str(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(',')
                .map(|(a, b)| (a.trim().parse().unwrap_or(0), b.trim().parse().unwrap_or(0)))
        })
        .collect()
}

pub fn part_1(points: &mut Vec<(i32, i32)>) -> u64 {
    let len = points.len();
    points.sort_unstable();

    let mut w = 0;
    for r in 0..len {
        if r == 0
            || r == len - 1
            || points[r].0 != points[r - 1].0
            || points[r].0 != points[r + 1].0
        {
            points[w] = points[r];
            w += 1;
        }
    }
    points.truncate(w);

    let mut max_area: u64 = 0;
    let n = points.len();
    for i in 0..n {
        let (x1, y1) = points[i];
        for j in i + 1..n {
            let (x2, y2) = points[j];
            let w = x1.abs_diff(x2) as u64 + 1;
            let h = y1.abs_diff(y2) as u64 + 1;
            let area = w * h;
            if area > max_area {
                max_area = area;
            }
        }
    }
    max_area
}
