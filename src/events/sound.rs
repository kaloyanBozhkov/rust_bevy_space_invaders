use bevy::prelude::*;

#[derive(Clone)]
pub enum SoundOperation {
    PLAY,
}

#[derive(Clone)]
pub struct SoundEvent {
    pub op: SoundOperation,
    pub sound_file_name: String,
    pub playback_settings: PlaybackSettings,
}

impl Default for SoundEvent {
    fn default() -> Self {
        SoundEvent {
            sound_file_name: "".to_string(),
            op: SoundOperation::PLAY,
            playback_settings: PlaybackSettings {
                repeat: false,
                volume: 0.4,
                speed: 1.0,
            },
        }
    }
}

pub fn sound_manager(
    mut ev_score: EventReader<SoundEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for ev in ev_score.iter() {
        if ev.sound_file_name != "" {
            let sound = asset_server.load(format!("sounds/{}", ev.sound_file_name));
            audio.play_with_settings(sound, ev.playback_settings.clone());
        }
    }
}
