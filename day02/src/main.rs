use std::fs;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32)
}

fn parse_command(line: &str) -> Command {
    let mut parts = line.split(" ");
    match (parts.next().expect("command missing"), parts.next().expect("amount missing").parse::<i32>().expect("amount not i32")) {
        ("forward", a) => Command::Forward(a),
        ("down", a) => Command::Down(a),
        ("up", a) => Command::Up(a),
        _ => panic!("invalid command")
    }
}

fn main() {
    let input = fs::read_to_string("../input/input-02-full.txt").expect("Problem reading input.");
    let commands: Vec<Command> = input.lines().map(|line| parse_command(line)).collect();
    println!("part 1: {}", get_part1(&commands)); // correct answer: 2215080
    println!("part 2: {}", get_part2(&commands)); // correct answer: 1864715580
}

fn get_part1(commands: &[Command]) -> i32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for command in commands {
        match command {
            Command::Forward(a) => x += a,
            Command::Down(a) => y += a,
            Command::Up(a) => y -= a,
        }
    }
    x * y
}

fn get_part2(commands: &[Command]) -> i32 {
    let mut aim: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for command in commands {
        match command {
            Command::Forward(a) => {
                x += a;
                y += aim * a;
            },
            Command::Down(a) => aim += a,
            Command::Up(a) => aim -= a,
        }
    }
    x * y
}
