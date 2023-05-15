use std::vec::Vec;
use rand::Rng;
use rand::seq::SliceRandom;
use coffee::graphics::{Color, Frame, Mesh, Shape, Rectangle, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

const DEFAULT_VERTICES: u32 = 3;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 800.0;

fn main() -> Result<()> {
    ChaosGame::run(WindowSettings {
        title: String::from("Chaos Game"),
        size: (SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
        resizable: false, fullscreen: false, maximized: false,
    })
}

struct Vertex {
    pub x: f32, pub y: f32,
    pub width: f32, pub height: f32,
    pub color: Color,
}

impl Vertex {
    pub fn draw(&self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        mesh.fill(Shape::Rectangle(Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }), self.color);
        mesh.draw(&mut frame.as_target());
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            x: rand::thread_rng().gen_range(0.0..SCREEN_WIDTH),
            y: rand::thread_rng().gen_range(0.0..SCREEN_HEIGHT),
            width: 1.0, height: 1.0, color: Color::WHITE,
        }
    }
}

struct ChaosGame {
    vertices: Vec<Vertex>,
    last_tracer: Vertex,
    initial_frame_drawn: bool,
}

impl Game for ChaosGame {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<ChaosGame> {
        Task::succeed(|| ChaosGame {
            vertices: (0..DEFAULT_VERTICES).map(|_| Vertex { width: 10.0, height: 10.0, color: Color::GREEN, ..Default::default() }).collect(),
            last_tracer: Default::default(),
            initial_frame_drawn: false,
        })
    }

    fn update(&mut self, _window: &Window) {
        let origin = &self.last_tracer;
        let target = self.vertices.choose(&mut rand::thread_rng())
            .expect("No random vertex could be selected from the vertices list.");

        self.last_tracer = Vertex {
            x: (origin.x + target.x) / 2.0,
            y: (origin.y + target.y) / 2.0,
            ..Default::default()
        };
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        if !self.initial_frame_drawn {
            frame.clear(Color::BLACK);

            for vertex in self.vertices.iter() {
                vertex.draw(frame);
            }

            self.initial_frame_drawn = true;
        }

        self.last_tracer.draw(frame);
    }
}