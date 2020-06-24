use gdnative::api::AnimationPlayer;

pub trait AnimationPlayerExt {
    fn play_default(&mut self, animation_name: &str);
}

impl AnimationPlayerExt for AnimationPlayer {
    fn play_default(&mut self, animation_name: &str) {
        self.play(animation_name.into(), -1., 1., false)
    }
}
