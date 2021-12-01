use std::fs;

fn main() {
    println!("-- start --");

    let input = fs::read_to_string("../input/input-01-full.txt").expect("Problem reading input.");
    let depths: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

    part1(depths);
    part2(depths);

    println!("--  end  --");
}

fn part1(depths: Vec<u32>){

    let mut first: bool = true;
    let mut increases: u32 = 0;
    let mut previous: u32 = 0;

    for depth in depths {
        if !first && depth > previous {
            increases += 1;
        }
        first = false;
        previous = depth;
    }

    println!("part 1: increases = {}", increases);
}

fn part2(depths: Vec<u32>){

    let mut index: i32 = -1;

    let mut increases: u32 = 0;

    let mut prev: u32 = 0;
    let mut prev2: u32 = 0;

    let mut sum: u32 = 0;
    let mut prevsum: u32 = 0;

    for depth in depths {
        index += 1;

        if index >= 2 {
            sum = depth + prev + prev2;
        }

        if index >= 3 && sum > prevsum {
            increases += 1;
        }

        prev2 = prev;
        prev = depth;

        prevsum = sum;
    }

    println!("part 2: increases = {}", increases);
}
