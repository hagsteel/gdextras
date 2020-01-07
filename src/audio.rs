//! Audio helper
//! 
use gdnative::*;
use std::collections::HashMap;
use std::hash::Hash;

use crate::{gd_err, some_or_bail};

/// Convenience storage for AudioStreams.
pub struct SoundBank<T> {
    inner: HashMap<T, AudioStream>,
}

impl<T: Eq + Hash> SoundBank<T> {
    pub fn insert(&mut self, key: T, path: &str) {
        let stream = some_or_bail!(load_audio_stream(path.into()), "Failed to load audio stream: {}", path);
        self.inner.insert(key, stream);
    }

    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn get(&self, key: T) -> Option<AudioStream> {
        match self.inner.get(&key) {
            Some(s) => Some(s.clone()),
            None => None
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
pub fn play_audio_stream(mut owner: Node, stream: AudioStream) {
    let audio_player = Instance::<AudioPlayer>::new();
    let player_node = *audio_player.base();

    unsafe {
        owner.add_child(Some(player_node), false);
        let _ = audio_player
            .script()
            .map(|player| {
                player.play_sound(stream);
            })
            .map_err(|e| {
                gd_err!("Failed to play sound: {:?}", e);
            });
    }
}

// -----------------------------------------------------------------------------
//     - Audio player -
// -----------------------------------------------------------------------------
#[derive(Debug, NativeClass)]
#[inherit(Node)]
pub struct AudioPlayer {
    audio_stream_player: Option<AudioStreamPlayer>,
    should_loop: bool,
}

#[methods]
impl AudioPlayer {
    pub fn _init(_owner: Node) -> Self {
        Self {
            audio_stream_player: None,
            should_loop: false,
        }
    }

    fn connect_signals(&self, owner: Node) -> Option<()> {
        let mut audio_node = self.audio_stream_player?;
        unsafe {
            let obj = &owner.to_object();
            let res = audio_node.connect(
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

        Some(())
    }

    #[export]
    pub fn _ready(&mut self, mut owner: Node) {
        godot_print!("{:?}", "Audio getting ready");
        let audio_stream_player = AudioStreamPlayer::new();

        unsafe {
            owner.add_child(Some(audio_stream_player.to_node()), false);
            self.audio_stream_player = match owner.get_child(0) {
                Some(player) => player.cast::<AudioStreamPlayer>(),
                None => {
                    gd_err!("Failed to get audio stream player");
                    return;
                }
            };

            let audio_node = some_or_bail!(
                owner.get_child(0),
                "No audio stream player. Add a child node of this of the type `AudioStreamPlayer`"
            );

            let mut audio_stream_player = some_or_bail!(
                audio_node.cast::<AudioStreamPlayer>(),
                "Failed to cast node to `AudioStreamPlayer`"
            );

            audio_stream_player.stop();
            self.audio_stream_player = Some(audio_stream_player);
            self.connect_signals(owner);
        }
    }

    pub fn play_sound(&self, audio_stream: AudioStream) {
        godot_print!("{:?}", "play sound called");
        let mut audio_stream_player = some_or_bail!(
            self.audio_stream_player,
            "No audio stream player assigned to this node (check node names)"
        );

        unsafe {
            audio_stream_player.set_stream(Some(audio_stream));
            audio_stream_player.play(0.0);
        }
    }

    #[export]
    pub fn on_sound_finished(&self, mut owner: Node) {
        godot_print!("{:?}", "sound finished");

        unsafe {
            if self.should_loop {
                // Play again
                self.audio_stream_player.map(|mut player| player.play(0.0));
            } else {
                self.audio_stream_player.map(|mut player| player.stop());
                owner.queue_free();
            }
        }
    }
}

unsafe impl Send for AudioPlayer {}
