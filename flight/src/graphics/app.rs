use networking::statekeeping::state::State;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

pub struct App {
    pub gl: GlGraphics,
    pub state: Option<State>,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0., 0., 0., 0.];

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BLACK, gl);
        });

    }
}
