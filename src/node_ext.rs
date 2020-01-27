use gdnative::{GodotObject, Instance, MapMut, NativeClass, Node, UserData, Camera, Variant};
use crate::gd_err;

pub trait NodeExt: GodotObject + Clone {
    fn get_and_cast<U: GodotObject>(&self, path: &str) -> Option<U>;

    fn instance_map<U, V, F, T>(&self, path: &str, f: F)
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

    fn with_script<U, V, F>(self, f: F)
    where
        V: UserData<Target = U> + MapMut,
        U: NativeClass<Base = Self, UserData = V>,
        F: FnMut(&mut U, Self),
    {
            match Instance::<U>::try_from_base(self) {
                Some(instance) => {
                    if let Err(e) = instance.map_mut(f) {
                        gd_err!("{:?}", e);
                    }
                }
                None => gd_err!("failed to get instance"),
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
    }
}

node_ext!(Node);
node_ext!(Camera);

// impl NodeExt for Node {
//     fn get_and_cast<U: GodotObject>(&self, path: &str) -> Option<U> {
//         unsafe {
//             let node = self.get_node(path.into())?;
//             node.cast::<U>()
//         }
//     }

//     fn deferred(&mut self, func_name: &str, args: &[Variant]) {
//         unsafe {
//             self.call_deferred(func_name.into(), args);
//         }
//     }
// }

// impl NodeExt for Camera {
//     fn get_and_cast<U: GodotObject>(&self, path: &str) -> Option<U> {
//         unsafe {
//             let node = self.get_node(path.into())?;
//             node.cast::<U>()
//         }
//     }

//     fn deferred(&mut self, func_name: &str, args: &[Variant]) {
//         unsafe {
//             self.call_deferred(func_name.into(), args);
//         }
//     }
// }
