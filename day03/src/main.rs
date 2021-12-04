use std::fs;

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
}

fn get_part1(input: &BitGrid) -> u32
{
    let height = input.height();
    let mut gamma: u32 = 0;
    for index in 0..input.width() {
        gamma = (gamma << 1) | if input.bitvecs.iter().fold(0, |sum, row| sum + if row[index] {1} else {0}) >= height / 2 {1} else {0};
    }
    gamma * (!gamma & ((1 << input.width()) - 1))
}
