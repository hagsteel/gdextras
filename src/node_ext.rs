use gdnative::GodotObject;
use gdnative::Node;


pub trait NodeExt {
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
