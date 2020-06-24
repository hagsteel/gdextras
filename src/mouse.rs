use gdnative::api::Input;

pub fn hide_mouse() {
    let input = Input::godot_singleton();
    input.set_mouse_mode(Input::MOUSE_MODE_HIDDEN);
}
