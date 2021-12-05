// based on parasyte's "minimal-winit" example from his excellent "pixels" crate: https://github.com/parasyte/pixels/tree/main/examples/minimal-winit

#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::fs;
use std::cmp;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = SIZE as u32;
const HEIGHT: u32 = SIZE as u32;
const BOX_SIZE: i16 = 64;

struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,

    frame_count: u32,
    lines: Vec<Line>,
    grid: Grid,
    line_index: usize
}

fn main() -> Result<(), Error> {

    let lines: Vec<Line> = fs::read_to_string("../input/input-05-full.txt").expect("Problem reading input.")
        .lines().map(|l| Line::parse(l)).collect();

    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Advent of Code 2021 --- Day 5: Hydrothermal Venture --- Visualization 1")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new(lines);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new(lines: Vec<Line>) -> Self {
        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
            frame_count: 0,
            lines,
            grid: Grid::new(),
            line_index: 0
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        while self.line_index < self.lines.len() {
            self.lines[self.line_index].add_to_grid(&mut self.grid, true);
            self.line_index += 1;
        }
        
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, frame: &mut [u8]) {
        
        self.frame_count += 1;

        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let rgba = match self.grid.cells[i] {
                0 => continue,
                1 => [0x00, 0x00, 0xff, 0xff],
                2 => [0x00, 0xff, 0x00, 0xff],
                3 => [0xff, 0xff, 0x00, 0xff],
                _ => [0xff, 0xff, 0xff, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}



// ================================== Day 5 Solution Code ==================================

const SIZE: usize = 1000; // assuming all coordinates are in the range 0..=999, will verify in parsing


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
        let mut i = raw.split(',').map(|s| {
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
    fn new() -> Grid {
        Grid {
            cells: vec![0; SIZE * SIZE]
        }
    }
    
    fn increment_along_vector(&mut self, mut x: i32, mut y: i32, dx: i32, dy: i32, count: i32) {
        for _ in 0..count {
            let index = (y * (SIZE as i32) + x) as usize;
            self.cells[index] += 1;
            x += dx;
            y += dy;
        }
    }
}
