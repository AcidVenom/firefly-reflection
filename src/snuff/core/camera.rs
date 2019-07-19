use crate::snuff::core::Transform;

pub struct Camera {
    transform: Transform,
    projection: nalgebra_glm::Mat4,
    dimensions: nalgebra_glm::Vec2,
    fov: f32,
    near_plane: f32,
    far_plane: f32,
    is_dirty: bool,
    is_orthographic: bool,
}

impl Camera {
    //---------------------------------------------------------------------------------------------------
    pub fn new() -> Camera {
        let default_aspect = 720.0 / 1280.0;

        Camera {
            transform: Transform::new(),
            projection: nalgebra_glm::identity(),
            dimensions: nalgebra_glm::vec2(5.0, 5.0 * default_aspect),
            fov: 90.0,
            near_plane: 0.01,
            far_plane: 100.0,
            is_dirty: true,
            is_orthographic: true,
        }
    }

    //---------------------------------------------------------------------------------------------------
    fn mark_dirty(&mut self) {
        self.is_dirty = true
    }

    //---------------------------------------------------------------------------------------------------
    fn clean(&mut self) {
        self.is_dirty = false
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_orthographic(&mut self, orthographic: bool) -> &mut Camera {
        self.is_orthographic = orthographic;
        self.mark_dirty();

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }

    //---------------------------------------------------------------------------------------------------
    pub fn projection(&mut self) -> nalgebra_glm::Mat4 {
        if self.is_dirty {
            self.projection = if self.is_orthographic {
                let half_size = self.dimensions * 0.5;
                nalgebra_glm::ortho_lh(
                    -half_size.x,
                    half_size.x,
                    -half_size.y,
                    half_size.y,
                    self.near_plane,
                    self.far_plane,
                )
            } else {
                nalgebra_glm::perspective_fov_lh(
                    self.fov,
                    self.dimensions.x,
                    self.dimensions.y,
                    self.near_plane,
                    self.far_plane,
                )
            }
        }

        self.projection
    }

    //---------------------------------------------------------------------------------------------------
    pub fn view(&mut self) -> nalgebra_glm::Mat4 {
        self.transform.world_to_local()
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_orthographic_size_both(&mut self, size: &nalgebra_glm::Vec2) -> &mut Camera {
        self.dimensions = *size;

        self.set_orthographic(true);

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_orthographic_size_both_f(&mut self, width: f32, height: f32) -> &mut Camera {
        self.dimensions = nalgebra_glm::vec2(width, height);

        self.set_orthographic(true);

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_orthographic_size(&mut self, size: f32, height_over_width: f32) -> &mut Camera {
        self.dimensions.x = size;
        self.dimensions.y = size * height_over_width;

        self.set_orthographic(true);

        self.mark_dirty();

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_perspective_size(&mut self, size: &nalgebra_glm::Vec2) -> &mut Camera {
        self.dimensions = *size;

        self.set_orthographic(false);

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_fov(&mut self, fov: f32) -> &mut Camera {
        self.fov = fov;

        self.set_orthographic(false);

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_near_plane(&mut self, near: f32) -> &mut Camera {
        self.near_plane = near;
        self.mark_dirty();

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_far_plane(&mut self, far: f32) -> &mut Camera {
        self.far_plane = far;
        self.mark_dirty();

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn fov(&self) -> f32 {
        self.fov
    }

    //---------------------------------------------------------------------------------------------------
    pub fn near_plane(&self) -> f32 {
        self.near_plane
    }

    //---------------------------------------------------------------------------------------------------
    pub fn far_plane(&self) -> f32 {
        self.far_plane
    }
}
