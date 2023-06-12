use networking::statekeeping::state::State;
use macroquad::prelude::*;

pub fn render(state: &mut State) {
    clear_background(BLACK);
    for planet in &state.planets {
        draw_circle(
            planet.position[0] as f32,
            planet.position[1] as f32,
            planet.radius as f32,
            WHITE,
        );
        // Print framerate
        draw_text(
            &format!("FPS: {}", get_fps()),
            100.0,
            100.0,
            20.0,
            WHITE,
        );
    }
}
