use std::collections::HashMap;

pub trait GameState
{
    fn on_enter(&mut self) -> ();
    fn on_leave(&mut self) -> ();
    fn update(&mut self, dt : f32) -> ();
    fn draw(&mut self, frame : &mut glium::Frame, dt : f32) -> ();
}

pub struct GameStateManager
{
    states : HashMap<String, Box<GameState>>,
    current_state : String
}

impl GameStateManager
{
    //---------------------------------------------------------------------------------------------------
    pub fn new() -> GameStateManager
    {
        GameStateManager
        {
            states: HashMap::new(),
            current_state: String::new()
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn add_state(&mut self, name : String, state : Box<GameState>) -> ()
    {
        if self.states.contains_key(&name)
        {
            println!("[GameStateManager] Attempted to add a state with key '{}', but it already exists, skipping", name);
            return;
        }

        self.states.insert(name.clone(), state);
    }

    //---------------------------------------------------------------------------------------------------
    pub fn switch(&mut self, name : String) -> ()
    {
        match self.get_current_state()
        {
            Some(s) => s.on_leave(),
            None => {}
        }

        match self.states.get_mut(&name)
        {
            Some(s) => 
            {
                s.on_enter();
                self.current_state = name.clone();
            },

            None => 
            {
                println!("[GameStateManager] Attempted to switch to state '{}', but it doesn't exist, staying in current state", name);
            }
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn update(&mut self, dt : f32) -> ()
    {
        match self.get_current_state()
        {
            Some(s) => s.update(dt),
            None => {}
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn draw(&mut self, frame : &mut glium::Frame, dt : f32) -> ()
    {
        let drawn_frame = match self.get_current_state()
        {
            Some(s) => { s.draw(frame, dt) },
            None => {}
        };
    }

    //---------------------------------------------------------------------------------------------------
    fn get_current_state(&mut self) -> Option<& mut GameState>
    {
        if !self.current_state.is_empty()
        {
            match self.states.get_mut(&self.current_state)
            {
                Some(s) => Some(s.as_mut()),
                None => None
            }
        }
        else
        {
            None
        }
    }
}