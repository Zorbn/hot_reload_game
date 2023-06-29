use std::time;

pub struct Game {
    instant: time::Instant,
}

impl Game {
    pub fn new() -> Self {
        Self {
            instant: time::Instant::now(),
        }
    }

    pub fn start(&mut self) {

    }

    pub fn update(&mut self) -> bool {
        let delta_time = self.instant.elapsed().as_secs_f32();
        println!("delta: {}s", delta_time);
        self.instant = time::Instant::now();

        true
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}