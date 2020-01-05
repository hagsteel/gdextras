use gdnative::{Input, InputEvent};

// -----------------------------------------------------------------------------
//     - Joystick -
// -----------------------------------------------------------------------------
pub trait Joystick {
    fn strength(&self, key: &str) -> f32;

    fn strength_mul(&self, key: &str, multiplier: f32) -> f32 {
        self.strength(key) * multiplier
    }

    fn is_pressed(&self, key: &str) -> bool;
}

impl Joystick for Input {
    fn strength(&self, key: &str) -> f32 {
        self.get_action_strength(key.into()) as f32
    }

    fn is_pressed(&self, key: &str) -> bool {
        self.is_action_pressed(key.into())
    }
}

impl Joystick for InputEvent {
    fn strength(&self, key: &str) -> f32 {
        self.get_action_strength(key.into()) as f32
    }

    fn is_pressed(&self, key: &str) -> bool {
        self.is_action_pressed(key.into())
    }
}
