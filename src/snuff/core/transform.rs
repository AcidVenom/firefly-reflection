use nalgebra_glm;

pub struct Transform {
    translation: nalgebra_glm::Vec3,
    anchor: nalgebra_glm::Vec3,
    scale: nalgebra_glm::Vec3,
    rotation: nalgebra_glm::Quat,
    local_to_world: nalgebra_glm::Mat4,
    world_to_local: nalgebra_glm::Mat4,
    is_dirty: bool,
}

impl Transform {
    //---------------------------------------------------------------------------------------------------
    pub fn new() -> Transform {
        Transform {
            translation: nalgebra_glm::vec3(0.0, 0.0, 0.0),
            anchor: nalgebra_glm::vec3(0.0, 0.0, 0.0),
            scale: nalgebra_glm::vec3(1.0, 1.0, 1.0),
            rotation: nalgebra_glm::quat_identity(),
            local_to_world: nalgebra_glm::identity(),
            world_to_local: nalgebra_glm::identity(),
            is_dirty: true,
        }
    }

    //---------------------------------------------------------------------------------------------------
    fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    //---------------------------------------------------------------------------------------------------
    fn clean(&mut self) {
        self.is_dirty = false;
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_translation(&mut self, t: &nalgebra_glm::Vec3) -> &mut Transform {
        self.translation = *t;
        self.mark_dirty();

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_translation_f(&mut self, x: f32, y: f32, z: f32) -> &mut Transform {
        self.set_translation(&nalgebra_glm::vec3(x, y, z))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_translation_2d(&mut self, t: &nalgebra_glm::Vec2) -> &mut Transform {
        self.set_translation(&nalgebra_glm::vec3(t.x, t.y, self.translation.z))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_translation_2d_f(&mut self, x: f32, y: f32) -> &mut Transform {
        self.set_translation(&nalgebra_glm::vec3(x, y, self.translation.z))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_anchor(&mut self, a: &nalgebra_glm::Vec3) -> &mut Transform {
        self.anchor = *a;
        self.mark_dirty();

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_anchor_f(&mut self, x: f32, y: f32, z: f32) -> &mut Transform {
        self.set_anchor(&nalgebra_glm::vec3(x, y, z))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_anchor_2d(&mut self, a: &nalgebra_glm::Vec2) -> &mut Transform {
        self.set_anchor(&nalgebra_glm::vec3(a.x, a.y, self.anchor.z))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_anchor_2d_f(&mut self, x: f32, y: f32) -> &mut Transform {
        self.set_anchor(&nalgebra_glm::vec3(x, y, self.anchor.z))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_scale(&mut self, s: &nalgebra_glm::Vec3) -> &mut Transform {
        self.scale = *s;
        self.mark_dirty();

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_scale_f(&mut self, x: f32, y: f32, z: f32) -> &mut Transform {
        self.set_scale(&nalgebra_glm::vec3(x, y, z))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_scale_2d(&mut self, s: &nalgebra_glm::Vec2) -> &mut Transform {
        self.set_scale(&nalgebra_glm::vec3(s.x, s.y, self.scale.z))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_scale_2d_f(&mut self, x: f32, y: f32) -> &mut Transform {
        self.set_scale(&nalgebra_glm::vec3(x, y, self.scale.z))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_rotation(&mut self, r: &nalgebra_glm::Quat) -> &mut Transform {
        self.rotation = *r;
        self.mark_dirty();

        self
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_rotation_euler(&mut self, r: &nalgebra_glm::Vec3) -> &mut Transform {
        let mut quat = nalgebra_glm::quat_identity();
        quat = nalgebra_glm::quat_rotate(&quat, r.x, &nalgebra_glm::vec3(1.0, 0.0, 0.0));
        quat = nalgebra_glm::quat_rotate(&quat, r.y, &nalgebra_glm::vec3(0.0, 1.0, 0.0));
        quat = nalgebra_glm::quat_rotate(&quat, r.z, &nalgebra_glm::vec3(0.0, 0.0, 1.0));

        self.set_rotation(&quat)
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_rotation_euler_f(&mut self, x: f32, y: f32, z: f32) -> &mut Transform {
        self.set_rotation_euler(&nalgebra_glm::vec3(x, y, z))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn set_orientation(&mut self, r: f32) -> &mut Transform {
        self.set_rotation_euler(&nalgebra_glm::vec3(0.0, 0.0, r))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn translation(&self) -> nalgebra_glm::Vec3 {
        self.translation
    }

    //---------------------------------------------------------------------------------------------------
    pub fn translation_2d(&self) -> nalgebra_glm::Vec2 {
        nalgebra_glm::vec2(self.translation.x, self.translation.y)
    }

    //---------------------------------------------------------------------------------------------------
    pub fn anchor(&self) -> nalgebra_glm::Vec3 {
        self.anchor
    }

    //---------------------------------------------------------------------------------------------------
    pub fn anchor_2d(&self) -> nalgebra_glm::Vec2 {
        nalgebra_glm::vec2(self.anchor.x, self.anchor.y)
    }

    //---------------------------------------------------------------------------------------------------
    pub fn scale(&self) -> nalgebra_glm::Vec3 {
        self.scale
    }

    //---------------------------------------------------------------------------------------------------
    pub fn scale_2d(&self) -> nalgebra_glm::Vec2 {
        nalgebra_glm::vec2(self.scale.x, self.scale.y)
    }

    //---------------------------------------------------------------------------------------------------
    pub fn rotation(&self) -> nalgebra_glm::Quat {
        self.rotation
    }

    //---------------------------------------------------------------------------------------------------
    pub fn rotation_euler(&self) -> nalgebra_glm::Vec3 {
        nalgebra_glm::quat_euler_angles(&self.rotation)
    }

    //---------------------------------------------------------------------------------------------------
    pub fn orientation(&self) -> f32 {
        self.rotation_euler().z
    }

    //---------------------------------------------------------------------------------------------------
    pub fn local_to_world(&mut self) -> nalgebra_glm::Mat4 {
        if !self.is_dirty {
            self.local_to_world
        } else {
            let mut m: nalgebra_glm::Mat4 = nalgebra_glm::identity();

            m = nalgebra_glm::translate(&m, &self.translation);
            m = nalgebra_glm::scale(&m, &self.scale);
            m *= nalgebra_glm::quat_to_mat4(&self.rotation);
            m = nalgebra_glm::translate(&m, &self.anchor);

            self.world_to_local = nalgebra_glm::inverse(&m);
            self.local_to_world = m;

            self.clean();

            self.local_to_world
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn world_to_local(&mut self) -> nalgebra_glm::Mat4 {
        if !self.is_dirty {
            self.world_to_local
        } else {
            self.local_to_world();
            self.world_to_local
        }
    }

    //---------------------------------------------------------------------------------------------------
    pub fn translate(&mut self, t: &nalgebra_glm::Vec3) -> &mut Transform {
        self.set_translation(&(self.translation() + t))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn translate_f(&mut self, x: f32, y: f32, z: f32) -> &mut Transform {
        self.translate(&nalgebra_glm::vec3(x, y, z))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn translate_2d(&mut self, t: &nalgebra_glm::Vec2) -> &mut Transform {
        self.translate(&nalgebra_glm::vec3(t.x, t.y, 0.0))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn translate_2d_f(&mut self, x: f32, y: f32) -> &mut Transform {
        self.translate(&nalgebra_glm::vec3(x, y, 0.0))
    }

    //---------------------------------------------------------------------------------------------------
    pub fn roll(&mut self, r: f32) -> &mut Transform {
        self.set_orientation(self.orientation() + r)
    }
}
