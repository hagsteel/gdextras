use gdnative::{GodotObject, Instance, MapMut, NativeClass, Node, UserData};

pub trait NodeExt: GodotObject + Clone {
    fn get_and_cast<U: GodotObject>(&self, path: &str) -> Option<U>;
}

impl NodeExt for Node {
    fn get_and_cast<U: GodotObject>(&self, path: &str) -> Option<U> {
        unsafe {
            let node = self.get_node(path.into())?;
            node.cast::<U>()
        }
    }
}

pub trait NodeExt2<T>: NodeExt
where
    T: GodotObject + Clone,
{
    fn instance_map<U, V, F>(&self, path: &str, f: F)
    where
        V: UserData<Target = U> + MapMut,
        U: NativeClass<Base = T, UserData = V>,
        F: FnMut(&mut U, T),
    {
        let node = self.get_and_cast::<U::Base>(path.into()).unwrap();
        let _ = Instance::<U>::try_from_base(node).unwrap().map_mut(f);
    }
}

impl<T> NodeExt2<T> for Node where T: GodotObject + Clone {}
