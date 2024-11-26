use macroquad::prelude::*;
use macroquad::ui::{self as ui};
use macroquad_particles::{self as particles};

struct ParticlesEditor {
    emitter: particles::Emitter,
    coords: Vec2,
}

impl ParticlesEditor {
    fn new() -> Self {
        let emitter = particles::Emitter::new(particles::EmitterConfig {
            local_coords: false,
            one_shot: false,
            emitting: true,
            lifetime: 0.4,
            lifetime_randomness: 0.5,
            explosiveness: 0.1,
            initial_direction: Vec2::new(0.0, 1.0),
            initial_direction_spread: 2.0,
            initial_velocity: 400.0,
            initial_velocity_randomness: 0.8,
            size: 10.0,
            size_randomness: 0.3,
            atlas: Some(particles::AtlasConfig::new(5, 1, 0..)),
            ..Default::default()
        });

        let coords = vec2(screen_width() / 2.0, screen_height() / 2.0);

        Self { emitter, coords }
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

        if ui::root_ui().button(None, "PUSH") {
            editor.emitter.config.size = 30.0;
        }

        editor.draw_emitter();
        next_frame().await
    }
}
