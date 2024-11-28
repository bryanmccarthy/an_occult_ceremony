use macroquad::prelude::*;
use macroquad::ui::{self as ui, hash};
use macroquad_particles::{self as particles};

mod presets;

struct ParticlesEditor {
    emitter: particles::Emitter,
    coords: Vec2,
}

impl ParticlesEditor {
    fn new() -> Self {
        let emitter = particles::Emitter::new(particles::EmitterConfig {
            ..Default::default()
        });

        let coords = vec2(screen_width() / 2.0, screen_height() / 2.0);

        Self { emitter, coords }
    }

    fn update_coords(&mut self) {
        self.coords = vec2(screen_width() / 2.0, screen_height() / 2.0);
    }

    fn draw_emitter(&mut self) {
        self.emitter.draw(self.coords);
    }
}

struct WindowResizeDetector {
    last_size: Vec2,
}

impl WindowResizeDetector {
    fn new() -> Self {
        Self {
            last_size: vec2(screen_width(), screen_height()),
        }
    }

    fn update(&mut self) {
        self.last_size = vec2(screen_width(), screen_height());
    }

    fn has_resized(&self) -> bool {
        screen_width() != self.last_size.x || screen_height() != self.last_size.y
    }
}

fn conf() -> Conf {
    Conf {
        window_title: "Particle Editor".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut editor = ParticlesEditor::new();
    let mut resizer_detector = WindowResizeDetector::new();

    loop {
        clear_background(BLACK);

        if resizer_detector.has_resized() {
            resizer_detector.update();
            editor.update_coords();
        }

        ui::widgets::Window::new(hash!(), vec2(10.0, 10.0), vec2(120.0, 200.0))
            .label("Presets")
            .ui(&mut ui::root_ui(), |ui| {
                if ui.button(None, "Default") {
                    editor.emitter.config = presets::default();
                }
                if ui.button(None, "Smoke") {
                    editor.emitter.config = presets::smoke();
                }
                if ui.button(None, "Fire") {
                    editor.emitter.config = presets::fire();
                }
                if ui.button(None, "Explosion") {
                    editor.emitter.config = presets::explosion();
                }
            });

        if ui::root_ui().button(None, "Log config") {
            println!("{:#?}", editor.emitter.config);
        }

        editor.draw_emitter();

        next_frame().await
    }
}
