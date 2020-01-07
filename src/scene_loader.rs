use crate::{gd_err, some_or_bail};
use gdnative::*;

#[macro_export]
macro_rules! with_scene_loader {
    ($node: ident, $e: expr) => {
        unsafe {
            match $node.get_node("/root/SceneLoader".into()) {
                Some(wd) => {
                    let n = wd.cast::<Node>().unwrap();
                    let _ = Instance::<SceneLoader>::try_from_base(n)
                        .unwrap()
                        .map_mut($e);
                }
                None => (),
            }
        }
    };
}


struct Loader {
    inner: ResourceInteractiveLoader,
}

impl Loader {
    pub fn new(path: &str) -> Option<Self> {
        let mut loader = ResourceLoader::godot_singleton();

        Some(Self {
            inner: loader.load_interactive(path.into(), "PackedScene".into())?,
        })
    }
}

impl Drop for Loader {
    fn drop(&mut self) {
        unsafe {
            self.inner.free();
        }
    }
}

unsafe impl Send for Loader {}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct SceneLoader {
    loader: Option<Loader>,
}

#[methods]
impl SceneLoader {
    pub fn _init(_owner: Node) -> Self {
        Self { loader: None }
    }

    pub fn change_scene(&mut self, mut owner: Node, path: &str) {
        self.loader = Loader::new(path);

        unsafe {
            owner.set_process(true);

            let tree = some_or_bail!(owner.get_tree(), "failed to get scene tree");
            let root = some_or_bail!(tree.get_root(), "failed to get root node");
            let mut current_scene = some_or_bail!(
                root.get_child(root.get_child_count() - 1),
                "failed to get current scene"
            );
            current_scene.queue_free();
        }
    }

    #[export]
    pub fn _ready(&self, mut owner: Node) {
        unsafe { owner.set_process(false) }
    }

    #[export]
    pub fn _process(&mut self, mut owner: Node, _delta: f64) {
        let loader = some_or_bail!(&mut self.loader, "failed to get loader");
        match loader.inner.poll() {
            Ok(()) => {
                let current = loader.inner.get_stage();
                let total = loader.inner.get_stage_count();
                self.update_progress(total, current);
            }
            Err(GodotError::FileEof) => {
                unsafe { owner.set_process(false) }
            }
            Err(e) => {
                unsafe { owner.set_process(false) }
                gd_err!("Error polling loader: {:?}", e);
                self.loader = None;
            }
        }
    }

    fn update_progress(&mut self, total: i64, current: i64) {
        godot_print!("{} / {}", current, total);
    }
}
