use coffee::graphics::{Color, Frame, Window, WindowSettings};
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

struct ChaosGame {
}

impl Game for ChaosGame {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<ChaosGame> {
        Task::succeed(|| ChaosGame {
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);
    }
}