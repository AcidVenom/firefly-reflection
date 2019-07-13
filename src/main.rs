#[macro_use]
extern crate glium;

mod snuff;

fn main()
{
    let mut window = snuff::core::Window::new(1280, 720, "Firefly - Reflection");
    let mut quad = snuff::gfx::Mesh::create_quad(window.gl_display(), true);

    window.exec();
}