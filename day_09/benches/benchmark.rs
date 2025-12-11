use std::hint::black_box;

use day_09::{parse_str, part_1, read_input};
use divan::Bencher;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_io(bencher: Bencher) {
    bencher.bench(|| black_box(read_input("src/input.txt")));
}

#[divan::bench]
fn bench_parse(bencher: Bencher) {
    bencher
        .with_inputs(|| read_input("src/input.txt"))
        .bench_values(|text| black_box(parse_str(&text)));
}

#[divan::bench]
fn bench_part_1(bencher: Bencher) {
    bencher
        .with_inputs(|| {
            let text = read_input("src/input.txt");
            parse_str(&text)
        })
        .bench_values(|mut data| part_1(black_box(&mut data)));
}
