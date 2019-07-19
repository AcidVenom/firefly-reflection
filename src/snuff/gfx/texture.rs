use std::io::Read;
use glium::texture;

pub struct Texture2D {
    texture: glium::texture::Texture2d,
    dimensions: nalgebra_glm::U16Vec2
}

type ImageData = Vec<u8>;

impl Texture2D {
    //---------------------------------------------------------------------------------------------------
    pub fn from_data(display : &glium::Display, data : &ImageData, width : u16, height : u16) -> Texture2D {
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(data, (width as u32, height as u32).into());
        Texture2D {
            texture: glium::texture::Texture2d::new(display, image).unwrap(),
            dimensions: nalgebra_glm::vec2(width, height)
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn from_image<'a>(display : &glium::Display, path : &'a str) -> Texture2D
    {
        let mut fin = std::fs::File::open(path).expect(&format!("[Texture2D] Could not open file '{}'", path)[..]);
        let mut data : Vec<u8> = vec![];

        fin.read_to_end(&mut data).expect(&format!("[Texture2D] Could not read image data from file '{}'", path)[..]);

        let image_data = image::load_from_memory(&data).unwrap().to_rgba();
        let dimensions = image_data.dimensions();

        Texture2D::from_data(display, &image_data.into_raw(), dimensions.0 as u16, dimensions.1 as u16)
    }

    //---------------------------------------------------------------------------------------------------
    pub fn texture(&self) -> &glium::texture::Texture2d {
        &self.texture
    }
}
