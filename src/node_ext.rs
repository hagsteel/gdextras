use gdnative::{GodotObject, Instance, MapMut, NativeClass, Node, UserData};

pub trait NodeExt : GodotObject + Clone
{
    fn get_and_cast<U: GodotObject>(&self, path: &str) -> Option<U>;

    fn instance_map<U, V, F, T>(&self, path: &str, f: F)
    where
        T: GodotObject + Clone,
        V: UserData<Target = U> + MapMut,
        U: NativeClass<Base = T, UserData = V>,
        F: FnMut(&mut U, T),
    {
        self.get_and_cast::<U::Base>(path.into()).map(|node| {
            let _ = Instance::<U>::try_from_base(node).unwrap().map_mut(f);
        });
    }
}


impl NodeExt for Node {
    fn get_and_cast<U: GodotObject>(&self, path: &str) -> Option<U> {
        unsafe {
            let node = self.get_node(path.into())?;
            node.cast::<U>()
        }
    }
}

