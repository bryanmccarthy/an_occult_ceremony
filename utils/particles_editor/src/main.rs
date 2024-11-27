use macroquad::prelude::*;
use macroquad::ui::{self as ui};
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

    fn update(&mut self) {
        self.coords = vec2(screen_width() / 2.0, screen_height() / 2.0);
    }

    fn draw_emitter(&mut self) {
        self.emitter.draw(self.coords);
    }
}

#[macroquad::main("particle editor")]
async fn main() {
    let mut editor = ParticlesEditor::new();

    loop {
        clear_background(BLACK);

        if ui::root_ui().button(None, "Smoke") {
            editor.emitter.config = presets::smoke();
        }

        if ui::root_ui().button(None, "Fire") {
            editor.emitter.config = presets::fire();
        }

        if ui::root_ui().button(None, "Explosion") {
            editor.emitter.config = presets::explosion();
        }

        editor.draw_emitter();

        editor.update();
        next_frame().await
    }
}
