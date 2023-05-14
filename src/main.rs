use std::vec::Vec;
use rand::Rng;
use rand::seq::SliceRandom;
use coffee::graphics::{Color, Frame, Mesh, Shape, Rectangle, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

const DEFAULT_VERTICES: u32 = 3;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

fn main() -> Result<()> {
    ChaosGame::run(WindowSettings {
        title: String::from("Chaos Game"),
        size: (SCREEN_WIDTH, SCREEN_HEIGHT),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })
}

#[derive(Clone)]
struct Point {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
}

impl Point {
    fn random(width: u32, height: u32, color: Color) -> Self {
        return Self {
            x: rand::thread_rng().gen_range(0..SCREEN_WIDTH),
            y: rand::thread_rng().gen_range(0..SCREEN_HEIGHT),
            width, height, color
        };
    }

    pub fn draw(&self, frame: &mut Frame) {
        let mut mesh = Mesh::new();
        mesh.fill(Shape::Rectangle(Rectangle {
            x: self.x as f32,
            y: self.y as f32,
            width: self.width as f32,
            height: self.height as f32,
        }), self.color);
        mesh.draw(&mut frame.as_target());
    }
}

impl Default for Point {
    fn default() -> Self {
        return Self::random(1, 1, Color::WHITE);
    }
}

struct ChaosGame {
    vertices: Vec<Point>,
    tracer_history: Vec<Point>,
}

impl Game for ChaosGame {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<ChaosGame> {
        Task::succeed(|| ChaosGame {
            vertices: (0..DEFAULT_VERTICES).map(|_| Point { width: 10, height: 10, color: Color::GREEN, ..Default::default() }).collect(),
            tracer_history: vec![Default::default()],
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        for vertex in self.vertices.iter() {
            vertex.draw(frame);
        }

        self.tracer_update();
        for point in self.tracer_history.iter() {
            point.draw(frame);
        }
    }
}

impl ChaosGame {
    fn tracer_update(&mut self) {
        let origin = self.tracer_history.last()
            .expect("At least one vertex should be defined!");
        let target = self.vertices.choose(&mut rand::thread_rng())
            .expect("No random vertex could be selected from the vertices list.");

        self.tracer_history.push(Point {
            x: (origin.x + target.x) / 2,
            y: (origin.y + target.y) / 2,
            ..Default::default()
        });
    }
}