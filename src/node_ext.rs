use crate::{gd_err, gd_panic};
use gdnative::{
    Camera, Camera2D, GodotObject, Instance, KinematicBody, KinematicBody2D, MapMut, NativeClass,
    Node, Node2D, Particles, Spatial, UserData, Variant,
};

pub trait NodeExt: GodotObject + Clone {
    fn get_and_cast<U: GodotObject>(&self, path: &str) -> Option<U>;

    fn get_and_map<U, V, F, T>(&self, path: &str, f: F)
    where
        T: GodotObject + Clone,
        V: UserData<Target = U> + MapMut,
        U: NativeClass<Base = T, UserData = V>,
        F: FnMut(&mut U, T),
    {
        self.get_and_cast::<U::Base>(path.into()).map(|node| {
            match Instance::<U>::try_from_base(node) {
                Some(instance) => {
                    if let Err(e) = instance.map_mut(f) {
                        gd_err!("{:?}", e);
                    }
                }
                None => gd_err!("failed to get instance"),
            }
        });
    }

    fn with_script<T, U, V, F>(self, f: F) -> T
    where
        V: UserData<Target = U> + MapMut,
        U: NativeClass<Base = Self, UserData = V>,
        F: FnMut(&mut U, Self) -> T,
    {
        match Instance::<U>::try_from_base(self) {
            Some(instance) => match instance.map_mut(f) {
                Ok(val) => val,
                Err(e) => {
                    gd_panic!("{:?}", e);
                }
            },
            None => {
                gd_panic!("failed to get instance");
            }
        }
    }

    fn deferred(&mut self, func_name: &str, args: &[Variant]);
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
