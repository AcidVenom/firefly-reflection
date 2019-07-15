use crate::snuff::core;

pub struct GameLoop {
    window: core::Window,
    game_state_manager: core::GameStateManager,
    frame_count: u32,
}

struct DeltaTime {
    new_time: std::time::Instant,
    new_dt: f32,
}

//---------------------------------------------------------------------------------------------------
fn calculate_delta_time(old_time: std::time::Instant) -> DeltaTime {
    let new_time = std::time::Instant::now();
    let new_dt = new_time.duration_since(old_time).as_millis() as f64 * 1e-3;

    DeltaTime {
        new_time: new_time,
        new_dt: new_dt as f32,
    }
}

impl GameLoop {
    //---------------------------------------------------------------------------------------------------
    pub fn new(window_width: u16, window_height: u16, window_title: &'static str, vsync : bool) -> GameLoop {
        GameLoop {
            window: core::Window::new(window_width, window_height, window_title, vsync),
            game_state_manager: core::GameStateManager::new(),
            frame_count: 0,
        }
    }

    //---------------------------------------------------------------------------------------------------
    fn draw(&mut self, dt: f32) {
        let mut target = self.window.begin_frame();

        self.game_state_manager.draw(&mut target, dt);

        self.window.end_frame(target);
    }

    //---------------------------------------------------------------------------------------------------
    fn tick(&mut self, dt: f32) {
        self.game_state_manager.update(dt);

        self.draw(dt);

        self.frame_count += 1;
    }

    //---------------------------------------------------------------------------------------------------
    pub fn exec(&mut self) {
        let mut old_time = std::time::Instant::now();
        let mut dt = 0.0;

        while self.window.process_events() {
            self.tick(dt);

            let deltas = calculate_delta_time(old_time);

            dt = deltas.new_dt;
            old_time = deltas.new_time;
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn window(&mut self) -> &mut core::Window {
        &mut self.window
    }

    //---------------------------------------------------------------------------------------------------
    pub fn game_state_manager(&mut self) -> &mut core::GameStateManager {
        &mut self.game_state_manager
    }

    //---------------------------------------------------------------------------------------------------
    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }
}
