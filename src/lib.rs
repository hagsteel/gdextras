pub mod controls;
pub mod movement;
pub mod mouse;

#[macro_export]
macro_rules! gd_unimplemented { 
    () => ({
        let line = std::line!();
        let file = std::file!();
        gdnative::godot_print!("{}:{} Rust: unimplemented!", file, line);
        std::unimplemented!()
    });
}

#[macro_export]
macro_rules! gd_err {
    ($($arg:tt)*) => ({
        let line = std::line!();
        let file = std::file!();
        let val: String = format!($($arg)*);
        gdnative::godot_error!("{}:{} {}", file, line, val);
    });
}


#[macro_export]
macro_rules! gd_panic {
    ($($arg:tt)*) => ({
        $crate::gd_err!($($arg)*);
        panic!($($arg)*);
    });
}

