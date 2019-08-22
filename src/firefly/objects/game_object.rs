use crate::snuff::gfx;
use crate::snuff::core;

pub struct RenderData<'a> {
    pub textures: Vec<&'a gfx::Texture2D>,
    pub transform: &'a mut core::Transform
}

pub trait GameObject {
    fn render_data(&mut self) -> RenderData;
}