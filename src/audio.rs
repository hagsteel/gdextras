use super::some_or_bail;
use gdnative::*;

/// Load an audio file.
/// E.g
/// ```
/// load_audio_stream("res://sfx/ping.wav");
/// ```
pub fn load_audio_stream(path: &str) -> Option<AudioStream> {
    let mut loader = ResourceLoader::godot_singleton();
    loader
        .load(path.into(), "AudioStream".into(), false)?
        .cast::<AudioStream>()
}

// -----------------------------------------------------------------------------
//     - Audio player -
// -----------------------------------------------------------------------------
#[derive(NativeClass)]
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
    pub fn _ready(&mut self, owner: Node) {
        unsafe {
            let audio_node = some_or_bail!(
                owner.get_node("AudioStreamPlayer".into()),
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

    pub fn play_sound(&self, audio_stream: Option<AudioStream>) {
        let mut audio_stream_player = some_or_bail!(
            self.audio_stream_player,
            "No audio stream player assigned to this node (check node names)"
        );

        if audio_stream.is_none() {
            // globals.created_audio.remove(globals.created_audio.find(self));
            return;
        }

        unsafe {
            audio_stream_player.set_stream(audio_stream);
            audio_stream_player.play(0.0);
        }
    }

    #[export]
    pub fn on_sound_finished(&self, mut owner: Node) {
        godot_print!("{:?}", "sound finished");

        unsafe {
            if self.should_loop {
                // Play again
                self.audio_stream_player.unwrap().play(0.0);
            } else {
                self.audio_stream_player.unwrap().stop();
                owner.queue_free();
                // globals.created_audio.remove(globals.created_audio.find(self));
            }
        }
    }
}

unsafe impl Send for AudioPlayer {}

// -----------------------------------------------------------------------------
//     - Audio singleton -
// -----------------------------------------------------------------------------
#[derive(NativeClass)]
#[inherit(Node)]
pub struct AudioSingleton {}

#[methods]
impl AudioSingleton {
    pub fn _init(_owner: Node) -> Self {
        Self {}
    }

    #[export]
    pub fn _ready(&self, _owner: Node) {
        godot_print!("Audio singleton ready");
    }
}
