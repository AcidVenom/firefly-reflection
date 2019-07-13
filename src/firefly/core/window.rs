extern crate glium;

pub struct Window
{
    client_width : u16,
    client_height : u16,
    events_loop : glium::glutin::EventsLoop,
    gl_display : glium::Display
}

use glium::glutin;

impl Window
{
    //---------------------------------------------------------------------------------------------------
    pub fn new(width : u16, height : u16, title : &'static str) -> Window
    {
        let events_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new()
            .with_dimensions((width as u32, height as u32).into())
            .with_title(title);

        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &events_loop).unwrap();

        Window{
            client_width: width, 
            client_height: height,
            events_loop: events_loop,
            gl_display: display
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn client_width(&self) -> u16
    {
        self.client_width
    }

    //---------------------------------------------------------------------------------------------------
    pub fn client_height(&self) -> u16
    {
        self.client_height
    }

    //---------------------------------------------------------------------------------------------------
    fn process_events(&mut self) -> bool
    {
        let mut closed = false;
        self.events_loop.poll_events(|evt|
        {
            match evt
            {
                glutin::Event::WindowEvent { event, .. } => match event
                {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => ()
                },
                _ => ()
            }
        });

        !closed
    }

    //---------------------------------------------------------------------------------------------------
    fn swap_buffers(&mut self) -> bool
    {
        let res = self.gl_display.swap_buffers();
        return res.is_ok();
    }

    //---------------------------------------------------------------------------------------------------
    pub fn exec(&mut self) -> u8
    {
        while self.process_events()
        {
            if !self.swap_buffers()
            {
                println!("[Window] Could not swap buffers!");
                return 1;
            }
        }

        return 0;
    }
}