use std::fs;

fn main() {
    println!("-- start --");
    part1();
    println!("--  end  --");
}

fn part1(){
    let input = fs::read_to_string("../input/input-01-full.txt").expect("Problem reading input.");
    let depths: Vec<u32> = input.lines().map(|line| line.parse().unwrap()).collect();

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


