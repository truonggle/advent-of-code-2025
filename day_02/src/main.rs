use std::{fs, io::Result};

fn main() -> Result<()> {
    let mut data = fs::read("src/input.txt")?;
    data.push(b',');

    let mut first: u64 = 0;
    let mut second: u64 = 0;
    let mut parsing_first = true;
    let mut total_sum: u64 = 0;

    for &byte in &data {
        match byte {
            b'0'..=b'9' => {
                let digit = (byte - b'0') as u32;
                if parsing_first {
                    first = first * 10 + digit as u64;
                } else {
                    second = second * 10 + digit as u64;
                }
            }
            b'-' => parsing_first = false,
            b',' => {
                //total_sum += part_1(&first, &second);
                total_sum += part_2(&first, &second);
                first = 0;
                second = 0;
                parsing_first = true;
            }
            _ => {}
        }
    }
    print!("{total_sum}");
    Ok(())
}

fn part_2(lo: &u64, hi: &u64) -> u64 {
    let mut total_sum = 0u64;

    for num in *lo..=*hi {
        let str = num.to_string();
        let len = str.len();

        for sub_len in 1..=len / 2 {
            if len % sub_len != 0 {
                continue;
            }
            let repetitions = len / sub_len;
            let sub_str = &str[..sub_len];
            let repeated = sub_str.repeat(repetitions);

            if repeated == str {
                total_sum += num;
                break;
            }
        }
    }
    total_sum
}

fn part_1(lo: &u64, hi: &u64) -> u64 {
    let mut ans: u64 = 0;
    let mut len = num_digits(*lo);
    if len % 2 != 0 {
        len += 1;
    }

    while len <= num_digits(*hi) {
        let half_len = len / 2;
        let pow10 = 10u64.pow(half_len as u32);
        let start = pow10 / 10;
        let end = pow10 - 1;
        for h in start..=end {
            let n = h * pow10 + h;
            if n > *hi {
                break;
            }
            if n >= *lo {
                ans += n;
            }
        }
        len += 2;
    }
    ans
}

fn num_digits(mut n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}
