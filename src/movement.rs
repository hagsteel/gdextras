use gdnative::{KinematicBody, KinematicBody2D, Node2D, Vector2, Vector3};

pub const UP2D: Vector2 = Vector2::new(0.0, 1.0);
pub const UP3D: Vector3 = Vector3::new(0.0, 1.0, 0.0);

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
pub trait MoveAndSlide2D {
    /// Default implementation of move_and_slide.
    fn move_and_slide_default(&mut self, velocity: Vector2, up: Vector2) -> Vector2;

    /// Apply gravity
    fn apply_gravity(&self, gravity: f32, velocity: &mut Vector2);
}

/// Move and slide for 3D nodes
pub trait MoveAndSlide3D {
    /// Default implementation of move_and_slide.
    fn move_and_slide_default(&mut self, velocity: Vector3, up: Vector3) -> Vector3;

    /// Apply gravity
    fn apply_gravity(&self, gravity: f32, velocity: &mut Vector3);
}

// -----------------------------------------------------------------------------
//     - Kinetmatic body 2D -
// -----------------------------------------------------------------------------
impl MoveAndSlide2D for KinematicBody2D {
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

    fn apply_gravity(&self, gravity: f32, velocity: &mut Vector2) {
        if !unsafe { self.is_on_floor() } {
            velocity.y += 1.0 * gravity;
        }
    }
}

// -----------------------------------------------------------------------------
//     - Kinetmatic body 3D -
// -----------------------------------------------------------------------------
impl MoveAndSlide3D for KinematicBody {
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

pub struct MouseAim2D {}

// -----------------------------------------------------------------------------
//     - Rotation 3D -
// -----------------------------------------------------------------------------
pub struct Rotation3D {
    pub aim_direction: Option<Vector3>,
}
