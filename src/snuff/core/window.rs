pub struct Window
{
    client_width : u16,
    client_height : u16,
    events_loop : glium::glutin::EventsLoop,
    gl_display : glium::Display
}

use glium::glutin;
use glium::Surface;

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
    pub fn gl_display(&mut self) -> &mut glium::Display
    {
        &mut self.gl_display
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
    pub fn process_events(&mut self) -> bool
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
    pub fn begin_frame(&mut self) -> glium::Frame
    {
        let mut target = self.gl_display.draw();
        target.clear_color(0.1, 0.33, 1.0, 1.0);

        target
    }

    //---------------------------------------------------------------------------------------------------
    pub fn end_frame(&mut self, target : glium::Frame) -> ()
    {
        target.finish().unwrap();
    }
}