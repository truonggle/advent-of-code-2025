use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

fn main() {
    let mut lines = read_lines("src/input.txt").unwrap();
    //part_1(&mut lines).unwrap();
    part_2(&mut lines).unwrap();
}

fn part_2(lines: &mut Lines<BufReader<File>>) -> Result<u64> {
    let mut total_fresh_ingredient_ids = 0u64;
    let fresh_ingredient_id_ranges = read_and_merge_ranges(lines)?;
    for range in fresh_ingredient_id_ranges {
        total_fresh_ingredient_ids += range.1 - range.0 + 1;
    }

    print!("{total_fresh_ingredient_ids}");
    Ok(total_fresh_ingredient_ids)
}

fn part_1(lines: &mut Lines<BufReader<File>>) -> Result<u32> {
    let mut fresh_ingredient_count = 0u32;
    let fresh_ingredient_id_ranges = read_and_merge_ranges(lines)?;

    for line in lines {
        let ingredient_id = line?.trim().parse().unwrap();
        if check_fresh_ingredient(ingredient_id, &fresh_ingredient_id_ranges) {
            fresh_ingredient_count += 1;
        }
    }
    print!("{fresh_ingredient_count}");
    Ok(fresh_ingredient_count)
}

fn check_fresh_ingredient(ingredient_id: u64, fresh_ingredient_id_ranges: &[(u64, u64)]) -> bool {
    let mut i = 0;
    while i < fresh_ingredient_id_ranges.len() {
        let fresh_ingredient_id = fresh_ingredient_id_ranges[i];
        if ingredient_id > fresh_ingredient_id.0 && ingredient_id < fresh_ingredient_id.1 {
            return true;
        }
        i += 1;
    }
    false
}

fn read_and_merge_ranges(lines: &mut Lines<BufReader<File>>) -> Result<Vec<(u64, u64)>> {
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for line in lines {
        let line = line?;
        if line.trim().is_empty() {
            break;
        }

        if let Some((a, b)) = line.split_once('-') {
            let start = a.trim().parse().unwrap();
            let end = b.trim().parse().unwrap();
            ranges.push((start, end));
        }
    }

    if ranges.is_empty() {
        return Ok(ranges);
    }

    ranges.sort_unstable_by_key(|r| r.0);
    let mut merged = vec![ranges[0]];
    for (start, end) in ranges.into_iter().skip(1) {
        let last_range = merged.last_mut().unwrap();
        if start <= last_range.1 {
            last_range.1 = last_range.1.max(end);
        } else {
            merged.push((start, end));
        }
    }

    Ok(merged)
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
