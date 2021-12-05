use std::fs;
use std::collections::HashSet;

const BINGO_SIZE: usize = 5; // bingo boards are assumed square, so this is both width & height

struct BingoSystem {
    numbers: Vec<u32>, // sequence of numbers to be called out
    boards: Vec<BingoBoard> // all boards
}

struct BingoBoard {
    cells: Vec<u32>, // row-first, left to right top to bottom, number per cell
    claimed: HashSet<usize>, // indices into cells vector of numbers that have been called out
    disable_future_winning: bool // when true, future calls to is_winner will return false
}

impl BingoSystem {
    fn new() -> BingoSystem {
        BingoSystem {
            numbers: Vec::<u32>::new(),
            boards: Vec::<BingoBoard>::new()
        }
    }

    fn parse(raw_input: &str) -> BingoSystem {
        let mut bs = BingoSystem::new();

        // go through each input line, tracking its index -- TODO: find better way
        let mut line_index: isize = -1;
        for line in raw_input.lines() {
            line_index += 1;

            // process the first line as the numbers list
            if line_index == 0 {
                bs.numbers = line.split(',').map(|s| s.parse().expect("u32 call number")).collect();
                continue;
            }

            // start new board on blank line between each
            if 0 == (line_index as usize - 1) % (BINGO_SIZE + 1) {
                bs.boards.push(BingoBoard::new());
                continue;
            }

            // push board row
            let last_board_index = bs.boards.len() - 1;
            let board = &mut bs.boards[last_board_index];
            for n in line.split(' ').map(|s| s.parse::<u32>()) {
                match n {
                    Ok(x) => board.cells.push(x),
                    _ => continue
                }
            }
        }
        bs
    }

    fn get_part_answers(&mut self) -> (u32, u32) {
        let mut found_first: bool = false;
        let mut part1: u32 = 0;
        let mut part2: u32 = 0;
        for num in &self.numbers[..] {
            for board in &mut self.boards[..] {
                board.claim(*num);
                if board.is_winner() {
                    board.disable_future_winning = true;
                    part2 = num * board.sum_of_unmarked();
                    if !found_first {
                        part1 = part2;
                        found_first = true;
                    }
                }
            }
        }
        (part1, part2)
    }
}

impl BingoBoard {
    fn new() -> BingoBoard {
        BingoBoard {
            cells: Vec::<u32>::new(),
            claimed: HashSet::<usize>::new(),
            disable_future_winning: false
        }
    }

    // marks a number as claimed (if on this board)
    fn claim(&mut self, num: u32) {
        for index in 0..self.cells.len() {
            if self.cells[index] == num {
                self.claimed.insert(index);
            }
        }
    }

    fn is_winner(&self) -> bool {
        // can't be a winner if the number claimed is less than the bingo size, or we disabled future winning
        if self.claimed.len() < BINGO_SIZE || self.disable_future_winning {
            return false;
        }

        // otherwise try every column and row
        for n in 0..BINGO_SIZE {
            if self.is_seq_claimed(0, n, 1, 0) || self.is_seq_claimed(n, 0, 0, 1) {
                return true
            }
        }

        false
    }

    fn is_seq_claimed(&self, mut x: usize, mut y: usize, dx: usize, dy: usize) -> bool {
        for _ in 0..BINGO_SIZE {
            let index = y * BINGO_SIZE + x;
            if !self.claimed.contains(&index) {
                return false
            }
            x += dx;
            y += dy;
        }
        true
    }

    fn sum_of_unmarked(&self) -> u32 {
        let mut sum = 0;
        for index in 0..self.cells.len() {
            if !self.claimed.contains(&index) {
                sum += self.cells[index];
            }
        }
        sum
    }
}

fn main() {
    let mut bs = BingoSystem::parse(&fs::read_to_string("../input/input-04-full.txt").expect("Problem reading input."));
    let (part1, part2) = bs.get_part_answers();
    println!("part 1: {}", part1); // 33348
    println!("part 2: {}", part2); // 8112
}
