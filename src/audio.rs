//! Audio helper
//! Example usage:
//!
//!```rust
//! // lib.rs
//! fn init(handle: init::InitHandle) {
//!     handle.add_class::<gdextras::audio::Player>();
//! }
//!
//! // world.rs
//! use gdextras::audio::{play_audio_stream, SoundBank}
//!
//! #[derive(NativeClass)]
//! #[inherit(Node2D)]
//! pub struct World {
//!    sfx_map: SoundBank<Sound>,
//! }
//!
//! #[methods]
//! impl World {
//!     pub fn _init(_owner: Node2D) -> Self {
//!         Self {
//!             sfx_map: SoundBank::new(),
//!         }
//!     }
//!
//!     #[export]
//!     pub fn _ready(&mut self, owner: Node2D) {
//!         self.sfx_map.insert(Sound::Gunshot, "res://sfx/boink.wav");
//!         self.sfx_map.insert(Sound::Rifle, "res://sfx/blip.wav");
//!     }
//!
//!     pub fn play_audio(&self, owner: Node2D, sound: Sound) -> Option<()> {
//!         let stream = self.sfx_map.get(sound)?;
//!         unsafe {
//!             play_audio_stream(owner.to_node(), stream);
//!         }
//!
//!         Some(())
//!     }
//! }
//! ```
use gdnative::*;
use std::collections::HashMap;
use std::hash::Hash;

use crate::{gd_err, some_or_bail};

/// Convenience storage for AudioStreams.
pub struct SoundBank<T> {
    inner: HashMap<T, AudioStream>,
}

unsafe impl<T> Send for SoundBank<T> {}
unsafe impl<T> Sync for SoundBank<T> {}

impl<T: Eq + Hash> SoundBank<T> {
    pub fn insert(&mut self, key: T, path: &str) {
        let stream = some_or_bail!(
            load_audio_stream(path.into()),
            "Failed to load audio stream: {}",
            path
        );
        self.inner.insert(key, stream);
    }

    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, key: &T) -> Option<AudioStream> {
        match self.inner.get(key) {
            Some(s) => Some(s.clone()),
            None => None,
        }
    }
}

/// Load an audio file.
/// E.g
/// ```ignore
/// load_audio_stream("res://sfx/ping.wav");
/// ```
pub fn load_audio_stream(path: &str) -> Option<AudioStream> {
    let mut loader = ResourceLoader::godot_singleton();
    loader
        .load(path.into(), "AudioStream".into(), false)?
        .cast::<AudioStream>()
}

/// Play an audio stream.
/// The audio stream will free and remove it self once done playing.
pub fn play_audio_stream(mut owner: Node, stream: AudioStream) {
    let audio_player = Instance::<AudioPlayer>::new();
    let player_node = *audio_player.base();

    unsafe {
        owner.add_child(Some(player_node.to_node()), false);
    }

    let _ = audio_player
        .map(|player, node| {
            player.play_sound(node, stream);
        })
        .map_err(|e| {
            gd_err!("Failed to play sound: {:?}", e);
        });
}

// -----------------------------------------------------------------------------
//     - Audio player -
// -----------------------------------------------------------------------------
/// Audio player node.
/// Attach this script to an audio stream player, and it can be set to loop 
#[derive(NativeClass)]
#[inherit(AudioStreamPlayer)]
pub struct AudioPlayer {
    #[property(path = "base/Loop")]
    should_loop: bool,
}

#[methods]
impl AudioPlayer {
    pub fn _init(_owner: AudioStreamPlayer) -> Self {
        Self { should_loop: false }
    }

    fn connect_signals(&self, mut owner: AudioStreamPlayer) {
        unsafe {
            let obj = &owner.to_object();
            let res = owner.connect(
                "finished".into(),
                Some(*obj),
                "on_sound_finished".into(),
                VariantArray::new(),
                0,
            );

            if let Err(e) = res {
                godot_print!("failed to connect audio signal: {:?}", e);
            }
        }
    }

    #[export]
    pub fn _ready(&mut self, mut owner: AudioStreamPlayer) {
        unsafe {
            owner.stop();
        }
        self.connect_signals(owner);
    }

    fn play_sound(&self, mut owner: AudioStreamPlayer, audio_stream: AudioStream) {
        unsafe {
            owner.set_stream(Some(audio_stream));
            owner.play(0.0);
        }
    }

    #[export]
    pub fn on_sound_finished(&self, mut owner: AudioStreamPlayer) {
        unsafe {
            if self.should_loop {
                // Play again
                owner.play(0.0);
            } else {
                owner.stop();
                owner.queue_free();
            }
        }
    }
}
