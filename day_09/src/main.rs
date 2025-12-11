use day_09::{parse_str, part_1, read_input};

fn main() {
    let text = read_input("src/input.txt");
    let mut data = parse_str(&text);
    let result = part_1(&mut data);
    print!("{result}");
}
