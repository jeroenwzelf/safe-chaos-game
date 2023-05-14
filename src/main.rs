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
        resizable: false,
        fullscreen: false,
        maximized: false,
    })
}

struct Point {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Point {
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

impl Default for Point {
    fn default() -> Self {
        return Self {
            x: rand::thread_rng().gen_range(0.0..SCREEN_WIDTH),
            y: rand::thread_rng().gen_range(0.0..SCREEN_HEIGHT),
            width: 1.0, height: 1.0, color: Color::WHITE,
        };
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
            vertices: (0..DEFAULT_VERTICES).map(|_| Point { width: 10.0, height: 10.0, color: Color::GREEN, ..Default::default() }).collect(),
            tracer_history: vec![Default::default()],
        })
    }

    fn update(&mut self, _window: &Window) {
        let origin = self.tracer_history.last()
            .expect("At least one vertex should be defined!");
        let target = self.vertices.choose(&mut rand::thread_rng())
            .expect("No random vertex could be selected from the vertices list.");

        self.tracer_history.push(Point {
            x: (origin.x + target.x) / 2.0,
            y: (origin.y + target.y) / 2.0,
            ..Default::default()
        });
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        for vertex in self.vertices.iter() {
            vertex.draw(frame);
        }

        for point in self.tracer_history.iter() {
            point.draw(frame);
        }
    }
}