pub mod firefly;

fn main()
{
    let mut window = firefly::core::Window::new(1280, 720, "Firefly - Reflection");
    window.exec();
}