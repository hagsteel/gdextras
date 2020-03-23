use crate::gd_err;
use gdnative::{
    Camera, Camera2D, Control, GodotObject, Instance, KinematicBody, KinematicBody2D, Label,
    MapMut, NativeClass, Node, Node2D, Particles, Spatial, UserData, Variant, Vector2, Viewport,
};

pub trait NodeExt: GodotObject + Clone + std::fmt::Debug {
    fn get_and_map<U, V, F, T, X>(&self, path: &str, f: F) -> Option<T>
    where
        X: GodotObject + Clone,
        V: UserData<Target = U> + MapMut,
        U: NativeClass<Base = X, UserData = V>,
        F: FnMut(&mut U, X) -> T,
    {
        let node = self.get_and_cast::<U::Base>(path.into())?;
        let instance = Instance::<U>::try_from_base(node)?;
        match instance.map_mut(f) {
            Ok(val) => Some(val),
            Err(e) => {
                gd_err!("{:?}", e);
                None
            }
        }
    }

    fn with_script<T, U, V, F>(self, f: F) -> Option<T>
    where
        V: UserData<Target = U> + MapMut,
        U: NativeClass<Base = Self, UserData = V>,
        F: FnMut(&mut U, Self) -> T,
    {
        match Instance::<U>::try_from_base(self) {
            Some(instance) => match instance.map_mut(f) {
                Ok(val) => Some(val),
                Err(e) => {
                    gd_err!("{:?}", e);
                    None
                }
            },
            None => None,
        }
    }

    fn deferred(&mut self, func_name: &str, args: &[Variant]);
    fn get_and_cast<U: GodotObject>(&self, path: &str) -> Option<U>;

    fn root(&self) -> Viewport {
        self.get_and_cast::<Viewport>("/root").unwrap()
    }
}

macro_rules! node_ext {
    ($type: ident) => {
        impl NodeExt for $type {
            fn get_and_cast<U: GodotObject>(&self, path: &str) -> Option<U> {
                unsafe {
                    let node = self.get_node(path.into())?;
                    node.cast::<U>()
                }
            }

            fn deferred(&mut self, func_name: &str, args: &[Variant]) {
                unsafe {
                    self.call_deferred(func_name.into(), args);
                }
            }
        }
    };
}

node_ext!(Node);
node_ext!(Node2D);
node_ext!(Camera);
node_ext!(Camera2D);
node_ext!(KinematicBody);
node_ext!(KinematicBody2D);
node_ext!(Particles);
node_ext!(Spatial);
node_ext!(Label);
node_ext!(Control);

pub trait NodeExt2D: GodotObject + Clone {
    fn canvas_mouse_pos(&self) -> Vector2;
    fn global_mouse_pos(&self) -> Vector2;
}

macro_rules! node_ext2d {
    ($type: ident) => {
        impl NodeExt2D for $type {
            fn canvas_mouse_pos(&self) -> Vector2 {
                unsafe { self.to_canvas_item().get_global_mouse_position() }
            }

            fn global_mouse_pos(&self) -> Vector2 {
                unsafe { self.get_global_mouse_position() }
            }
        }
    };
}

node_ext2d!(Node2D);
node_ext2d!(Camera2D);
node_ext2d!(KinematicBody2D);
node_ext2d!(Control);
node_ext2d!(Label);
