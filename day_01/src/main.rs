use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
    str::Chars,
};

fn main() {
    let mut dial_value: u32 = 50;
    let mut zero_count: u32 = 0;
    let mut rotation: Chars;
    let mut dir: char;
    let mut dist: u32;
    let mut dial_offset: u32;
    let mut zeros_during_rotation: u32;

    if let Ok(lines) = read_lines("src/input.txt") {
        'rotations: for line in lines.map_while(Result::ok) {
            rotation = line.chars();
            dir = rotation.next().unwrap();
            dist = rotation.as_str().parse().unwrap();
            dial_offset = dist % 100;
            zeros_during_rotation = dist / 100; // For Part 2

            zero_count = zero_count + zeros_during_rotation; // For Part 2
            match dir {
                'L' => match dial_offset.cmp(&dial_value) {
                    Ordering::Less => {
                        dial_value = dial_value - dial_offset;
                        continue 'rotations;
                    }
                    Ordering::Greater => {
                        // This whole `if` statement is for Part 2
                        if dial_value != 0 {
                            zero_count = zero_count + 1;
                        }
                        dial_value = 100 - (dial_offset - dial_value);
                        continue 'rotations;
                    }
                    Ordering::Equal => {
                        zero_count = zero_count + 1;
                        dial_value = 0;
                        continue 'rotations;
                    }
                },
                'R' => match (dial_value + dial_offset).cmp(&100) {
                    Ordering::Less => {
                        dial_value = dial_value + dial_offset;
                        continue 'rotations;
                    }
                    Ordering::Greater => {
                        zero_count = zero_count + 1; // For Part 2
                        dial_value = (dial_value + dial_offset) - 100;
                        continue 'rotations;
                    }
                    Ordering::Equal => {
                        zero_count = zero_count + 1;
                        dial_value = 0;
                        continue 'rotations;
                    }
                },
                _ => break,
            }
        }
    }
    print!("{zero_count}");
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
