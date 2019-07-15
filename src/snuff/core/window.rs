pub struct Window {
    client_width: u16,
    client_height: u16,
    events_loop: glium::glutin::EventsLoop,
    display: glium::Display,
}

use glium::glutin;
use glium::Surface;

impl Window {
    //---------------------------------------------------------------------------------------------------
    pub fn new(width: u16, height: u16, title: &'static str. vsync : bool) -> Window {
        let events_loop = glutin::EventsLoop::new();
        let wb = glutin::WindowBuilder::new()
            .with_dimensions((u32::from(width), u32::from(height)).into())
            .with_title(title);

        let cb = glutin::ContextBuilder::new().with_vsync(vsync);
        let display = glium::Display::new(wb, cb, &events_loop).unwrap();

        Window {
            client_width: width,
            client_height: height,
            events_loop,
            display,
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn display(&mut self) -> &mut glium::Display {
        &mut self.display
    }

    //---------------------------------------------------------------------------------------------------
    pub fn client_width(&self) -> u16 {
        self.client_width
    }

    //---------------------------------------------------------------------------------------------------
    pub fn client_height(&self) -> u16 {
        self.client_height
    }

    //---------------------------------------------------------------------------------------------------
    pub fn process_events(&mut self) -> bool {
        let mut closed = false;
        self.events_loop.poll_events(|evt| match evt {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => closed = true,
                _ => (),
            },
            _ => (),
        });

        !closed
    }

    //---------------------------------------------------------------------------------------------------
    pub fn begin_frame(&mut self) -> glium::Frame {
        let mut target = self.display.draw();
        target.clear_color(0.1, 0.33, 1.0, 1.0);

        target
    }

    //---------------------------------------------------------------------------------------------------
    pub fn end_frame(&mut self, target: glium::Frame) {
        match target.finish()
        {
            Ok(_) => {},
            Err(e) => println!("[Window] Could not swap buffers in 'end_frame' : {}", e)
        }
    }
}
