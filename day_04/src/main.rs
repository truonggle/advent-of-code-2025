use std::{fs, io::Result};

fn main() -> Result<()> {
    let text = fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&[u8]> = text.lines().map(|l| l.as_bytes()).collect();
    let total_paper_rolls = part_1(&lines);
    //let total_paper_rolls = part_2(&lines);
    print!("{total_paper_rolls}");
    Ok(())
}

fn part_2(lines: &[&[u8]]) -> usize {
    let remaining_paper_rolls: usize = lines.len() * lines[0].len();
    remaining_paper_rolls
}

fn part_1(lines: &[&[u8]]) -> usize {
    let height = lines.len();
    let width = lines[0].len();

    const DIRS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut paper_rolls = 0;
    for r in 0..height {
        for c in 0..width {
            if lines[r][c] != b'@' {
                continue;
            }
            let mut neighbors = 0u8;
            for &(dr, dc) in &DIRS {
                let rr = r as isize + dr;
                let cc = c as isize + dc;
                if rr >= 0
                    && rr < height as isize
                    && cc >= 0
                    && cc < width as isize
                    && lines[rr as usize][cc as usize] == b'@'
                {
                    neighbors += 1;
                    if neighbors >= 4 {
                        break;
                    }
                }
            }
            if neighbors < 4 {
                paper_rolls += 1;
            }
        }
    }
    paper_rolls
}
