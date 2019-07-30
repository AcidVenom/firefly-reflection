use crate::snuff::core;

pub struct GameLoop {
    window: core::Window,
    game_state_manager: core::GameStateManager,
    frame_count: u32,
}

struct DeltaTime {
    new_time: u64,
    new_dt: f32,
}

//---------------------------------------------------------------------------------------------------
fn calculate_delta_time(old_time: u64) -> DeltaTime {
    let new_time = time::precise_time_ns();
    let new_dt = (new_time - old_time) as f64 * 1e-9;

    DeltaTime {
        new_time,
        new_dt: new_dt as f32,
    }
}

impl GameLoop {
    //---------------------------------------------------------------------------------------------------
    fn set_working_directory() {
        let args: Vec<String> = std::env::args().collect();
        if args.len() > 1 {
            let working_dir = args[1].clone();
            assert!(
                std::env::set_current_dir(&working_dir).is_ok(),
                format!("[main] Invalid working directory '{}'", working_dir)
            );
            println!("[main] Set working directory to '{}'", working_dir);
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn new(
        window_width: u16,
        window_height: u16,
        window_title: &'static str,
        vsync: bool,
    ) -> GameLoop {
        GameLoop::set_working_directory();

        GameLoop {
            window: core::Window::new(window_width, window_height, window_title, vsync),
            game_state_manager: core::GameStateManager::new(),
            frame_count: 0,
        }
    }

    //---------------------------------------------------------------------------------------------------
    fn draw(&mut self, time: f32, dt: f32) {
        
    }

    //---------------------------------------------------------------------------------------------------
    fn tick(&mut self, time: f32, dt: f32) {
        let game_state_manager = &mut self.game_state_manager;
        let current_state = game_state_manager.get_current_state();

        let next_state = match current_state {
            Some(state) => { state.update(dt, &self.window) },
            _ => None
        };

        let mut target = self.window.begin_frame(time);
        let current_state = game_state_manager.get_current_state();

        match current_state {
            Some(state) => { state.draw(&mut target, dt); },
            None => {}
        };

        target.end();

        match next_state {
            Some(state_name) => game_state_manager.switch(&state_name[..]),
            _ => {}
        };

        self.frame_count += 1;
    }

    //---------------------------------------------------------------------------------------------------
    pub fn exec(&mut self) {
        let mut old_time = time::precise_time_ns();
        let mut dt = 0.0;
        let mut time = 0.0;

        while self.window.process_events() {
            self.tick(time, dt);

            let deltas = calculate_delta_time(old_time);

            dt = deltas.new_dt;
            old_time = deltas.new_time;

            time += dt;
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
