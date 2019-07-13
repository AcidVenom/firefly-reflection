#[macro_use]
extern crate glium;

mod snuff;

struct TestState
{
    id : u8
}

impl snuff::core::GameState for TestState
{
    fn on_enter(&mut self) -> ()
    {
        println!("Entering test state {}..", self.id);
    }

    fn on_leave(&mut self) -> ()
    {
        println!("Leaving test state {}..", self.id);
    }

    fn update(&mut self, dt : f32) -> ()
    {
        println!("Updating test state {}..", self.id);
    }

    fn draw(&mut self, frame : glium::Frame, dt : f32) -> glium::Frame
    {
        println!("Drawing test state {}..", self.id);
        frame
    }
}

fn main()
{
    let mut window = snuff::core::Window::new(1280, 720, "Firefly - Reflection");
    let mut game_state_manager = snuff::core::GameStateManager::new();

    let test_state_a = Box::new(TestState{ id: 0 });
    let test_state_b = Box::new(TestState{ id: 1 });

    game_state_manager.add_state(String::from("TestStateA"), test_state_a);
    game_state_manager.add_state(String::from("TestStateB"), test_state_b);

    game_state_manager.switch(String::from("TestStateA"));
    game_state_manager.switch(String::from("TestStateB"));

    let mut frame_count = 0;
    while window.process_events() && frame_count < 2
    {
        game_state_manager.update(0.0);

        let mut target = window.begin_frame();

        target = game_state_manager.draw(target, 0.0);

        window.end_frame(target);

        frame_count = frame_count + 1;
    }
}