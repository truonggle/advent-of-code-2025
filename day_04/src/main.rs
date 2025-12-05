use std::{fs, io::Result};

fn main() -> Result<()> {
    let text = fs::read_to_string("src/input.txt").unwrap();
    let lines: Vec<&[u8]> = text.lines().map(|l| l.as_bytes()).collect();
    //let total_paper_rolls = part_1(&lines);
    let total_paper_rolls = part_2(&lines);
    print!("{total_paper_rolls}");
    Ok(())
}

fn part_2(lines: &[&[u8]]) -> usize {
    let mut grid: Vec<Vec<u8>> = lines.iter().map(|row| row.to_vec()).collect();
    let mut paper_rolls_to_remove: usize = 0;
    let height = grid.len();
    let width = grid[0].len();

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

    loop {
        let mut to_remove = vec![];

        for r in 0..height {
            for c in 0..width {
                if grid[r][c] != b'@' {
                    continue;
                }
                let neighbors = DIRS
                    .iter()
                    .filter(|&&(dr, dc)| {
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        nr >= 0
                            && nr < height as isize
                            && nc >= 0
                            && nc < width as isize
                            && grid[nr as usize][nc as usize] == b'@'
                    })
                    .count();
                if neighbors < 4 {
                    to_remove.push((r, c));
                    paper_rolls_to_remove += 1;
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (r, c) in to_remove {
            grid[r][c] = b'.';
        }
    }
    paper_rolls_to_remove
}

fn part_1(grid: &[&[u8]]) -> usize {
    let height = grid.len();
    let width = grid[0].len();

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

    let mut paper_rolls_to_access = 0;
    for r in 0..height {
        for c in 0..width {
            if grid[r][c] != b'@' {
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
                    && grid[rr as usize][cc as usize] == b'@'
                {
                    neighbors += 1;
                    if neighbors >= 4 {
                        break;
                    }
                }
            }
            if neighbors < 4 {
                paper_rolls_to_access += 1;
            }
        }
    }
    paper_rolls_to_access
}
