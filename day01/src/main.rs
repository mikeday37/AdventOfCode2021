use std::fs;

fn main() {
    let input = fs::read_to_string("../input/input-01-full.txt").expect("Problem reading input.");
    let depths: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();
    println!("part 1: {}", count_increases_in_window(&depths, 1));
    println!("part 2: {}", count_increases_in_window(&depths, 3));
}

fn count_increases_in_window(values: &[u32], window_size: usize) -> u32 {
    let mut sum = 0;
    let mut prev = 0;
    let mut increases = 0;
    for start in 0..values.len() {
        sum += values[start];
        if start >= window_size {
            sum -= values[start - window_size];
            if sum > prev {
                increases += 1;
            }
        }
        prev = sum;
    }
    increases
}
