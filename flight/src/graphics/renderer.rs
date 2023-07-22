use common::data::state::State;
use macroquad::experimental::camera::mouse::Camera;
use macroquad::prelude::*;

use crate::data::client_state::ClientState;

use super::textures::Textures;

pub struct Renderer<'a> {
    camera: Camera,
    client_state: &'a ClientState,
    textures: &'a Textures,
}

impl<'a> Renderer<'a> {
    pub fn new(client_state: &'a ClientState, textures: &'a Textures) -> Self {
        Self {
            camera: Camera::default(),
            client_state,
            textures,
        }
    }

    pub fn render(&mut self, state: &State) {
        clear_background(BLACK);

        let mut objects = state.objects();

        let target_object_pos = objects.position(|p| p.id == self.client_state.center);

        let target_planet = if let Some(pos) = target_object_pos {
            objects.nth(pos)
        } else {
            None
        };

        let target = if let Some(t) = target_planet {
            vec2(t.position[0] as f32, t.position[1] as f32)
        } else {
            vec2(0.0, 0.0)
        };

        let wheel_val = mouse_wheel().1;
        let mouse_pos: Vec2 = mouse_position_local().into();

        self.camera.scale_wheel(mouse_pos, wheel_val, 1.1);
        self.camera.update(
            mouse_pos,
            is_mouse_button_down(MouseButton::Left),
        );

        let mut camera: Camera2D = (&self.camera).into();
        camera.target = target;

        set_camera(&camera);

        for planet in state.planets.iter() {
            let texture = self.textures.planets.get(&planet.object.texture);
            if texture.is_none() {
                log::error!("Texture {} not found", planet.object.texture);
                continue;
            }

            draw_texture_ex(
                *texture.unwrap(),
                planet.object.position[0] as f32 - planet.radius as f32,
                planet.object.position[1] as f32 - planet.radius as f32,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(planet.radius as f32 * 2., planet.radius as f32 * 2.)),
                    ..Default::default()
                },
            );
        }

        println!("FPS: {}", get_fps());
    }

    pub fn draw_splash(&self, splash_texture: Texture2D) {
        draw_texture(
            splash_texture,
            screen_width() / 2.0 - splash_texture.width() / 2.0,
            screen_height() / 2.0 - splash_texture.height() / 2.0,
            WHITE,
        );
    }
}
