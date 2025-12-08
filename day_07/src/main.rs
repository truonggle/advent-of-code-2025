use std::{fs::read_to_string, mem::swap};

fn main() {
    let s = read_to_string("src/input.txt").unwrap();
    //part_1(&s);
    part_2(&s);
}

fn part_2(input: &str) {
    let start_col = input.lines().find_map(|l| l.find('S')).unwrap();
    let mut beams = vec![(start_col, 0i8, 1u64)];
    let mut next = Vec::with_capacity(beams.len() * 2);

    for r in input.lines().map(|l| l.as_bytes()) {
        for (c, dir, count) in beams.drain(..) {
            match r.get(c as usize) {
                Some(b'^') => {
                    next.push((c - 1, -1, count));
                    next.push((c + 1, 1, count));
                }
                Some(_) => next.push((c, dir, count)),
                None => {}
            }
        }
        next.sort_unstable_by(|a, b| (a.0, b.1).cmp(&(b.0, b.1)));
        let mut merged: Vec<(usize, i8, u64)> = Vec::with_capacity(next.len());
        for (c, dir, count) in next.drain(..) {
            if let Some(last_element) = merged.last_mut() {
                if last_element.0 == c && last_element.1 == dir {
                    last_element.2 += count;
                    continue;
                }
            }
            merged.push((c, dir, count));
        }
        beams = merged;
    }
    let timelines: u64 = beams.iter().map(|b| b.2).sum();
    print!("{timelines}")
}

fn part_1(input: &str) {
    let mut beams = vec![input.find('S').unwrap()];
    let mut next = Vec::with_capacity(beams.len() * 2);
    let mut splits = 0;

    for r in input.lines().map(|l| l.as_bytes()) {
        for c in beams.drain(..) {
            match r.get(c) {
                Some(b'^') => {
                    splits += 1;
                    next.extend([c.wrapping_sub(1), c + 1]);
                }
                Some(_) => next.push(c),
                None => {}
            }
        }
        next.dedup();
        swap(&mut beams, &mut next);
    }
    print!("{splits}");
}
