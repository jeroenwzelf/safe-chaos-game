use std::vec::Vec;
use rand::Rng;
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

struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn random() -> Self {
        return Self {
            x: rand::thread_rng().gen_range(0..SCREEN_WIDTH),
            y: rand::thread_rng().gen_range(0..SCREEN_HEIGHT),
        };
    }
}

struct ChaosGame {
    points: Vec<Point>,
}

impl Game for ChaosGame {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<ChaosGame> {
        Task::succeed(|| ChaosGame {
            points: Vec::new(),
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);
        self.points.push(Point::random());
        for point in self.points.iter() {
            let mut mesh = Mesh::new();
            mesh.fill(Shape::Rectangle(Rectangle {
                x: point.x as f32,
                y: point.y as f32,
                width: 1.0,
                height: 1.0,
            }), Color::WHITE);
            mesh.draw(&mut frame.as_target());
        }
    }
}