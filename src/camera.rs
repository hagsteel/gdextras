use gdnative::{Camera, PhysicsServer, Variant, VariantArray, Vector2, Vector3};

pub trait CameraExt {
    fn pos_from_camera(&self, mouse_pos: Vector2, ray_length: f32) -> Option<Vector3>;
}

impl CameraExt for Camera {
    fn pos_from_camera(&self, mouse_pos: Vector2, ray_length: f32) -> Option<Vector3> {
        let from = unsafe { self.project_ray_origin(mouse_pos) };
        let to = from + unsafe { self.project_ray_normal(mouse_pos) } * ray_length;

        let rid = unsafe {
            let world = self.get_world().expect("failed to get world");
            world.get_space()
        };

        let mut phys_server = PhysicsServer::godot_singleton();
        let mut direct_state = phys_server
            .space_get_direct_state(rid)
            .expect("failed to get direct state");

        unsafe {
            let dict = direct_state.intersect_ray(
                from,                // From
                to,                  // To
                VariantArray::new(), // Ignored objects
                1,                   // Collision mask
                true,                // Collide with bodies
                false,               // Collide with areas
            );
            let pos_variant = dict.get(&Variant::from_godot_string(&"position".into()));
            if pos_variant.is_nil() {
                return None;
            }
            let pos = pos_variant.to_vector3();
            Some(pos)
        }
    }
}
