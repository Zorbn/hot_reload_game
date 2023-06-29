// use std::time;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

pub struct Game {
    // instant: time::Instant,
    window: minifb::Window,
    buffer: Vec<u32>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            // instant: time::Instant::now(),
            window: minifb::Window::new(
                "Test",
                WIDTH,
                HEIGHT,
                minifb::WindowOptions {
                    scale: minifb::Scale::X2,
                    scale_mode: minifb::ScaleMode::AspectRatioStretch,
                    resize: true,
                    ..Default::default()
                },
            )
            .expect("Failed to create window"),
            buffer: vec![0; WIDTH * HEIGHT],
        }
    }

    pub fn start(&mut self) {}

    pub fn update(&mut self) -> bool {
        if !self.window.is_open() || self.window.is_key_down(minifb::Key::Escape) {
            return false;
        }

        for i in 0..self.buffer.len() {
            self.buffer[i] = i as u32 * 27_962;
        }

        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .expect("Failed to update window with buffer");

        // let delta_time = self.instant.elapsed().as_secs_f32();
        // println!("delta: {}s", delta_time);
        // self.instant = time::Instant::now();

        true
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
