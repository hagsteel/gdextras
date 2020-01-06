use gdnative::{Input, InputEvent};

// -----------------------------------------------------------------------------
//     - Input extension -
// -----------------------------------------------------------------------------
pub trait InputExt {
    fn strength(&self, key: &str) -> f32;

    fn strength_mul(&self, key: &str, multiplier: f32) -> f32 {
        self.strength(key) * multiplier
    }

    fn is_pressed(&self, key: &str) -> bool;
}

impl InputExt for Input {
    fn strength(&self, key: &str) -> f32 {
        self.get_action_strength(key.into()) as f32
    }

    fn is_pressed(&self, key: &str) -> bool {
        self.is_action_pressed(key.into())
    }
}

// -----------------------------------------------------------------------------
//     - InputEvent extension -
// -----------------------------------------------------------------------------
pub trait InputEventExt {
    fn strength(&self, key: &str) -> f32;

    fn strength_mul(&self, key: &str, multiplier: f32) -> f32 {
        self.strength(key) * multiplier
    }

    fn is_pressed(&self, key: &str) -> bool;

    fn is_released(&self, key: &str) -> bool;
}

impl InputEventExt for InputEvent {
    fn strength(&self, key: &str) -> f32 {
        self.get_action_strength(key.into()) as f32
    }

    fn is_pressed(&self, key: &str) -> bool {
        self.is_action_pressed(key.into())
    }

    fn is_released(&self, key: &str) -> bool {
        self.is_action_released(key.into())
    }
}
