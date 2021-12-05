use std::fs;
use std::cmp;

const SIZE: usize = 999; // assuming all coordinates are in the range 0..=999, will verify in parsing

struct Line {
    a: Point,
    b: Point
}

struct Point {
    x: i32,
    y: i32
}

struct Grid {
    cells: Vec<i32> // count of overlapping lines at point, stored row major, left->right, top->bottom, width = SIZE
}

impl Line {
    fn parse(raw: &str) -> Line {
        let mut i = raw.split(" -> ").map(|s| Point::parse(s));
        Line {
            a: i.next().expect("a"),
            b: i.next().expect("b")
        }
    }

    fn add_to_grid(&self, grid: &mut Grid, include_diagonals: bool) {
        let dx = self.b.x - self.a.x;
        let dy = self.b.y - self.a.y;
        let d = cmp::max(dx.abs(), dy.abs());
        let dx = dx / d;
        let dy = dy / d;
        let c = d + 1;
        if include_diagonals || dx == 0 || dy == 0 {
            grid.increment_along_vector(self.a.x, self.a.y, dx, dy, c);
        }
    }
}


impl Point {
    fn parse(raw: &str) -> Point {
        let mut i = raw.split(",").map(|s| {
            let v = s.parse().expect("i32");
            if v >= SIZE as i32 {panic!("SIZE exceeded")}
            v
        });
        Point {
            x: i.next().expect("x"),
            y: i.next().expect("y")
        }
   }
}

impl Grid {
    fn parse(lines: &[Line], include_diagonals: bool) -> Grid {
        let mut grid = Grid {
            cells: vec![0; SIZE * SIZE]
        };
        for line in lines {
            line.add_to_grid(&mut grid, include_diagonals);
        }
        grid
    }

    fn increment_along_vector(&mut self, mut x: i32, mut y: i32, dx: i32, dy: i32, count: i32) {
        for _ in 0..count {
            let index = (y * (SIZE as i32) + x) as usize;
            self.cells[index] += 1;
            x += dx;
            y += dy;
        }
    }

    fn count_overlapped_points(&self) -> i32 {
        let mut sum = 0;
        for cell in &self.cells[..] {
            if *cell > 1 {
                sum += 1;
            }
        }
        sum
    }
}

fn main() {
    let lines: Vec<Line> = fs::read_to_string("../input/input-05-full.txt").expect("Problem reading input.")
        .lines().map(|l| Line::parse(l)).collect();
    let g1 = Grid::parse(&lines[..], false);
    let g2 = Grid::parse(&lines[..], true);
    println!("part 1: {}", g1.count_overlapped_points()); // 5632
    println!("part 2: {}", g2.count_overlapped_points()); // 22213
}
