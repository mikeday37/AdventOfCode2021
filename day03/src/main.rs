use std::fs;
use std::collections::HashSet;

struct BitGrid {
    bitvecs: Vec<Vec<bool>>
}

impl BitGrid {
    fn parse(raw_input: &str) -> BitGrid {
        let bitvecs: Vec<Vec<bool>> = raw_input.lines().map(|line| parse_bits(&line)).collect();
        
        // validate that input is present and all the same width
        if bitvecs.len() < 1 {
            panic!("there's less than one input line");
        }
        if bitvecs.len() > 1 {
            let width = bitvecs[0].len();
            for bv in &bitvecs {
                if width != bv.len() {
                    panic!("input has variable width");
                }
            }
        }

        BitGrid {bitvecs}
    }

    fn width(&self) -> usize {
        self.bitvecs[0].len()
    }

    fn height(&self) -> usize {
        self.bitvecs.len()
    }
}

fn parse_bits(line: &str) -> Vec<bool> {
    let mut bits = Vec::<bool>::new();
    for c in line.chars() {
        match c {
            '0' => bits.push(false),
            '1' => bits.push(true),
            _ => panic!("bad input")
        }
    }
    bits
}

fn main() {
    let input = BitGrid::parse(&fs::read_to_string("../input/input-03-full.txt").expect("Problem reading input."));
    println!("part 1: {}", get_part1(&input)); // 738234
    println!("part 2: {}", get_part2(&input)); // 3969126
}

fn get_part1(input: &BitGrid) -> u32 {
    let height = input.height();
    let mut gamma: u32 = 0;
    for index in 0..input.width() {
        gamma = (gamma << 1) | if input.bitvecs.iter().fold(0, |sum, row| sum + if row[index] {1} else {0}) >= height / 2 {1} else {0};
    }
    gamma * (!gamma & ((1 << input.width()) - 1))
}

fn get_part2(input: &BitGrid) -> u32 {
    let o2 = get_rating(input, true);
    let co2 = get_rating(input, false);
    o2 * co2
}

fn get_rating(input: &BitGrid, most_common: bool) -> u32 { // there's a bunch of derefs here that I don't understand the need for, yet
    let mut potentials: HashSet::<usize> = HashSet::from_iter(0..input.height());
    let mut index: isize = -1;
    while potentials.len() > 1 {
        index += 1;
        let ones: Vec<&usize> = potentials.iter().filter(|row| input.bitvecs[**row][index as usize]).collect();
        let zeroes: Vec<&usize> = potentials.iter().filter(|row| !input.bitvecs[**row][index as usize]).collect();
        let keep_value = if ones.len() == zeroes.len() {most_common} else {(ones.len() > zeroes.len()) ^ !most_common};
        let keep = if keep_value {ones} else {zeroes};
        potentials = HashSet::from_iter(keep.iter().map(|row| **row));
    }
    let rating_index = *potentials.iter().next().expect("nothing left");
    input.bitvecs[rating_index].iter().fold(0, |a, b| (a << 1) | if *b {1} else {0})
}
