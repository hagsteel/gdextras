use gdnative::{
    KinematicBody, KinematicBody2D, KinematicCollision2D, Node2D, Spatial, Vector2, Vector3,
    Viewport
};

use euclid::{Transform3D, UnknownUnit, Angle};
use euclid::Rotation3D as Rot3D;

type Transform3 = Transform3D<f32, UnknownUnit, UnknownUnit>;
type Rotation3 = Rot3D<f32, UnknownUnit, UnknownUnit>;

pub const UP_2D: Vector2 = Vector2::new(0.0, -1.0);
pub const DOWN_2D: Vector2 = Vector2::new(0.0, 1.0);

pub const UP_3D: Vector3 = Vector3::new(0.0, -1.0, 0.0);

// -----------------------------------------------------------------------------
//     - Move and slide -
// -----------------------------------------------------------------------------
/// Move and slide for 2D nodes
/// Example:
/// pub fn _unhandled_input(&mut self, owner: KinematicBody2D, event: InputEvent) {
///     let input = Input::godot_singleton();
///     self.velocity = Vector2::zero();
///     self.velocity.x -= input.strength_mul("ui_left", SPEED);
///     self.velocity.x += input.strength_mul("ui_right", SPEED);
///     self.velocity.y -= input.strength_mul("ui_up", SPEED);
///     self.velocity.y += input.strength_mul("ui_down", SPEED);
/// }
///
/// fn _physics_process(&mut self, mut owner, delta: f64) {
///     self.velocity = owner.move_and_slide_default(self.velocity, UP2D);
/// }
pub trait Move2D {
    /// Default implementation of move_and_slide.
    fn move_and_slide_default(&mut self, velocity: Vector2, up: Vector2) -> Vector2;

    // Default implementation for move_and_slide_with_snap
    fn move_and_slide_with_snap_default(
        &mut self,
        velocity: Vector2,
        snap: Vector2,
        up: Vector2,
    ) -> Vector2;

    /// Default implementation of move_and_collide.
    fn move_and_collide_default(&mut self, velocity: Vector2) -> Option<KinematicCollision2D>;
}

/// Move and slide for 3D nodes
pub trait Move3D {
    /// Default implementation of move_and_slide.
    fn move_and_slide_default(&mut self, velocity: Vector3, up: Vector3) -> Vector3;

    /// Apply gravity
    fn apply_gravity(&self, gravity: f32, velocity: &mut Vector3);
}

// -----------------------------------------------------------------------------
//     - Kinetmatic body 2D -
// -----------------------------------------------------------------------------
impl Move2D for KinematicBody2D {
    fn move_and_slide_default(&mut self, velocity: Vector2, up: Vector2) -> Vector2 {
        unsafe {
            let stop_on_slope = false;
            let max_slides = 4;
            let floor_max_angle = 0.785398;
            let infinite_inertia = true;

            self.move_and_slide(
                velocity,
                up,
                stop_on_slope,
                max_slides,
                floor_max_angle,
                infinite_inertia,
            )
        }
    }

    fn move_and_slide_with_snap_default(
        &mut self,
        velocity: Vector2,
        snap: Vector2,
        up: Vector2,
    ) -> Vector2 {
        unsafe {
            let stop_on_slope = false;
            let max_slides = 4;
            let floor_max_angle = 0.785398;
            let infinite_inertia = true;

            self.move_and_slide_with_snap(
                velocity,
                snap,
                up,
                stop_on_slope,
                max_slides,
                floor_max_angle,
                infinite_inertia,
            )
        }
    }

    fn move_and_collide_default(&mut self, velocity: Vector2) -> Option<KinematicCollision2D> {
        unsafe {
            self.move_and_collide(
                velocity, true,  // infinite intertia
                true,  // exclude raycsat shapes
                false, // test only
            )
        }
    }
}

// -----------------------------------------------------------------------------
//     - Kinetmatic body 3D -
// -----------------------------------------------------------------------------
impl Move3D for KinematicBody {
    fn move_and_slide_default(&mut self, velocity: Vector3, up: Vector3) -> Vector3 {
        unsafe {
            let stop_on_slope = false;
            let max_slides = 4;
            let floor_max_angle = 0.785398;
            let infinite_inertia = true;

            self.move_and_slide(
                velocity,
                up,
                stop_on_slope,
                max_slides,
                floor_max_angle,
                infinite_inertia,
            )
        }
    }

    fn apply_gravity(&self, gravity: f32, velocity: &mut Vector3) {
        if !unsafe { self.is_on_floor() } {
            velocity.y += 1.0 * gravity;
        }
    }
}

// -----------------------------------------------------------------------------
//     - Rotation 2D -
// -----------------------------------------------------------------------------
/// Rotation for 2D nodes.
/// Calling `look_at` is just shorthand for calling `look_at` on the selected Node2D.
///
/// `set_rotation` takes four positive values, so a combination of inpux axis strenghts
/// can be used.
pub struct Rotation2D {
    aim_direction: Option<Vector2>,
    owner: Node2D,
}

unsafe impl Send for Rotation2D {}

impl Rotation2D {
    pub fn new(owner: Node2D) -> Self {
        Self {
            aim_direction: None,
            owner,
        }
    }

    pub fn set_rotation(&mut self, left: f32, right: f32, up: f32, down: f32) {
        let dir = Vector2::new(-left + right, -up + down);
        if dir == Vector2::zero() {
            self.aim_direction = None;
            return;
        }
        self.aim_direction = Some(dir);
    }

    pub fn follow_mouse(&mut self) {
        unsafe {
            let mouse_pos = self.owner.get_global_mouse_position();
            self.owner.look_at(mouse_pos);
        }
    }

    pub unsafe fn update_rotation(&mut self) -> Option<()> {
        let aim_dir = self.aim_direction?;
        // let rot = self.owner.get_rotation();
        let new_rot = aim_dir.y.atan2(aim_dir.x) as f64;
        self.owner.set_rotation(new_rot);
        Some(())
    }
}

// -----------------------------------------------------------------------------
//     - Rotation 3D -
// -----------------------------------------------------------------------------

// Get the X, Y and Z direction from a transform
fn transform_to_x_y_z_direction(trans: Transform3) -> (Vector3, Vector3, Vector3) {
    let cols = trans.to_column_arrays();
    let v1 = Vector3::new(cols[0][0], cols[0][1], cols[0][2]);
    let v2 = Vector3::new(cols[1][0], cols[1][1], cols[1][2]);
    let v3 = Vector3::new(cols[2][0], cols[2][1], cols[2][2]);

    (v1, v2, v3)
}

pub struct Rotation3D {
    owner: Spatial,
    look_at_vec: Vector3,
}

fn vec2_to_3(vec2: Vector2, vec3: Vector3) -> Vector3 {
    Vector3::new(vec2.x, 0.0, vec2.y) + vec3
}

impl Rotation3D {
    pub fn new(owner: Spatial) -> Self {
        Self {
            owner,
            look_at_vec: Vector3::zero(),
        }
    }

    pub fn look_dir(&mut self, look_dir: Vector3) {
        if look_dir == Vector3::zero() {
            return
        }

        unsafe {
            let current_rot = self.owner.get_rotation();
            let cur_rot = Rotation3::around_y(Angle::radians(current_rot.y));

            let angle = Angle::radians(look_dir.x.atan2(look_dir.z));
            let new_rot = Rotation3::around_y(angle);

            // Implement smooth rotation

            // Instant rotation
            let new_transform = new_rot.to_transform();

            let (x, y, z) = transform_to_x_y_z_direction(new_transform);

            let mut current_transform = self.owner.get_transform();
            current_transform.basis.elements[0] = x;
            current_transform.basis.elements[1] = y;
            current_transform.basis.elements[2] = z;

            self.owner.set_transform(current_transform);
        }
    }
}
