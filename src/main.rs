use std::vec::Vec;
use rand::Rng;
use rand::seq::SliceRandom;
use coffee::graphics::{Color, Frame, Mesh, Shape, Rectangle, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

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
    pub fn random_vertex() -> Self {
        return Self::random(10, 10, Color::GREEN);
    }

    pub fn random_tracer() -> Self {
        return Self::random(1, 1, Color::WHITE);
    }

    pub fn tracer_at(x: u32, y: u32) -> Self {
        return Self { x, y, width: 1, height: 1, color: Color::WHITE };
    }

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

struct ChaosGame {
    vertices: Vec<Point>,
    tracer_history: Vec<Point>,
}

impl Game for ChaosGame {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<ChaosGame> {
        const DEFAULT_VERTICES: u32 = 3;
        Task::succeed(|| ChaosGame {
            vertices: (0..DEFAULT_VERTICES).map(|_| Point::random_vertex()).collect(),
            tracer_history: vec![Point::random_tracer()],
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
        let origin = self.tracer_history.last().expect("At least one vertex should be defined!");
        let target = self.vertices.choose(&mut rand::thread_rng()).expect("No random vertex could be selected from the vertices list.");

        let new_tracer = Point::tracer_at(
            (origin.x + target.x) / 2,
            (origin.y + target.y) / 2,
        );

        self.tracer_history.push(new_tracer);
    }
}