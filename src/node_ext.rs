use gdnative::api::{
    Camera, Camera2D, Control, KinematicBody, KinematicBody2D, Label, Node, Node2D, Particles,
    Spatial,
};
use gdnative::{GodotObject, MapMut, NativeClass, UserData, Vector2};

pub trait NodeExt: GodotObject + std::fmt::Debug {
    fn get_and_cast<T: GodotObject>(&self, path: &str) -> &T;

    fn with_script<T, U, V, F>(&self, f: F)
    where
        T: GodotObject,
        U: NativeClass<Base = T, UserData = V>,
        V: UserData<Target = U> + MapMut,
        F: FnOnce(&mut U, &T),
    {
        let node = self.cast::<T>().unwrap();
        node.cast_instance::<U>().and_then(|val| val.map_mut(f).ok());
    }
}

macro_rules! node_ext {
    ($type: ident) => {
        impl NodeExt for $type {
            fn get_and_cast<T: GodotObject>(&self, path: &str) -> &T {
                let node = self.get_node(path.into()).expect("node not found");

                unsafe {
                    let n = node.assume_safe();
                    n.cast::<T>().expect("invalid node type")
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

pub trait NodeExt2D: GodotObject {
    fn canvas_mouse_pos(&self) -> Vector2;
    fn global_mouse_pos(&self) -> Vector2;
}

macro_rules! node_ext2d {
    ($type: ident) => {
        impl NodeExt2D for $type {
            fn canvas_mouse_pos(&self) -> Vector2 {
                self.to_canvas_item().get_global_mouse_position()
            }

            fn global_mouse_pos(&self) -> Vector2 {
                self.get_global_mouse_position()
            }
        }
    };
}

node_ext2d!(Node2D);
node_ext2d!(Camera2D);
node_ext2d!(KinematicBody2D);
node_ext2d!(Control);
node_ext2d!(Label);
