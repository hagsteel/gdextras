use gdnative::{KinematicBody, KinematicBody2D, Node2D, Vector2, Vector3};

pub const UP2D: Vector2 = Vector2::new(0.0, 1.0);
pub const UP3D: Vector3 = Vector3::new(0.0, 1.0, 0.0);

// -----------------------------------------------------------------------------
//     - Move and slide -
// -----------------------------------------------------------------------------
pub trait MoveAndSlide2D {
    fn move_and_slide_default(&mut self, velocity: Vector2, up: Vector2) -> Vector2;
    fn apply_gravity(&self, gravity: f32, velocity: &mut Vector2);
}

pub trait MoveAndSlide3D {
    fn move_and_slide_default(&mut self, velocity: Vector3, up: Vector3) -> Vector3;
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
            return
        }
        self.aim_direction = Some(dir);
    }

    pub unsafe fn update_rotation(&mut self) -> Option<()> {
        let aim_dir = self.aim_direction?;
        // let rot = self.owner.get_rotation();
        let new_rot = aim_dir.y.atan2(aim_dir.x) as f64;
        self.owner.set_rotation(new_rot);
        Some(())
    }
}

pub struct MouseAim2D {
}

// -----------------------------------------------------------------------------
//     - Rotation 3D -
// -----------------------------------------------------------------------------
pub struct Rotation3D {
    pub aim_direction: Option<Vector3>,
}
