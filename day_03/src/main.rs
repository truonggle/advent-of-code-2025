use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

fn main() -> Result<()> {
    let mut total_max_joltage = 0u64;

    for line in read_lines("src/input.txt")? {
        let bank = line?;

        // Part 1
        /*if let Some((battery_1, battery_2)) = two_batteries_forming_largest_joltage(&bank) {
            total_max_joltage += (battery_1 as u64) * 10 + (battery_2 as u64);
        }*/

        // Part 2 (`k_batteries_forming_largest_joltage` can solve Part 1 by setting k = 2)
        if let Some(max_joltage) = k_batteries_forming_largest_joltage(&bank, 12) {
            total_max_joltage += max_joltage.parse::<u64>().unwrap();
        }
    }

    print!("{total_max_joltage}");
    Ok(())
}

// Part 2: Monotonic Decreasing Stack
fn k_batteries_forming_largest_joltage(bank: &str, k: usize) -> Option<String> {
    let batteries: Vec<u8> = bank
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .map(|b| b - b'0')
        .collect();

    if batteries.len() < k {
        return None;
    }

    let mut selected_batteries: Vec<u8> = Vec::with_capacity(k);
    let mut remaining_drops = batteries.len() - k;

    for &battery in &batteries {
        while remaining_drops > 0
            && !selected_batteries.is_empty()
            && *selected_batteries.last().unwrap() < battery
        {
            selected_batteries.pop();
            remaining_drops -= 1;
        }
        selected_batteries.push(battery);
    }

    let res: String = selected_batteries
        .into_iter()
        .take(k)
        .map(|battery| (battery + b'0') as char)
        .collect();
    Some(res)
}

// Part 1
fn two_batteries_forming_largest_joltage(bank: &str) -> Option<(u8, u8)> {
    let bytes = bank.as_bytes();
    let mut it = bytes.iter().filter(|&&b| b.is_ascii_digit());

    let mut left_battery = *it.next()? - b'0';
    let mut best_pair = None::<(u8, u8)>;

    for &byte in it {
        let battery = byte - b'0';
        if let Some((a, c)) = best_pair {
            if 10 * left_battery + battery > 10 * a + c {
                best_pair = Some((left_battery, battery));
            }
        } else {
            best_pair = Some((left_battery, battery));
        }
        if battery > left_battery {
            left_battery = battery;
        }
    }
    best_pair
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
