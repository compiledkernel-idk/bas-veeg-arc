use ahash::AHashMap;
use macroquad::audio::Sound;
use macroquad::prelude::*;

pub struct AudioMixer {
    channels: Vec<AudioChannel>,
    sound_library: AHashMap<String, Sound>,
    music_tracks: AHashMap<String, Sound>,
    master_volume: f32,
    sfx_volume: f32,
    music_volume: f32,
    current_music: Option<String>,
    ducking: DuckingState,
}

#[derive(Clone)]
pub struct AudioChannel {
    pub name: String,
    pub volume: f32,
    pub pan: f32,
    pub playing: Option<PlayingSound>,
}

#[derive(Clone)]
pub struct PlayingSound {
    pub sound_id: String,
    pub start_time: f64,
    pub looping: bool,
}

#[derive(Clone, Debug)]
pub struct DuckingState {
    pub active: bool,
    pub target_volume: f32,
    pub current_volume: f32,
    pub duration: f32,
    pub timer: f32,
}

impl AudioMixer {
    pub fn new() -> Self {
        let mut channels = Vec::new();
        for i in 0..16 {
            channels.push(AudioChannel {
                name: format!("channel_{}", i),
                volume: 1.0,
                pan: 0.0,
                playing: None,
            });
        }

        Self {
            channels,
            sound_library: AHashMap::new(),
            music_tracks: AHashMap::new(),
            master_volume: 1.0,
            sfx_volume: 1.0,
            music_volume: 0.8,
            current_music: None,
            ducking: DuckingState {
                active: false,
                target_volume: 1.0,
                current_volume: 1.0,
                duration: 0.0,
                timer: 0.0,
            },
        }
    }

    pub fn load_sound(&mut self, _id: String, _path: &str) {}

    pub fn load_music(&mut self, _id: String, _path: &str) {}

    pub fn play_sound(&mut self, sound_id: &str, volume: f32, pan: f32) -> Option<usize> {
        for (i, channel) in self.channels.iter_mut().enumerate() {
            if channel.playing.is_none() {
                channel.playing = Some(PlayingSound {
                    sound_id: sound_id.to_string(),
                    start_time: get_time(),
                    looping: false,
                });
                channel.volume = volume;
                channel.pan = pan;
                return Some(i);
            }
        }
        None
    }

    pub fn play_music(&mut self, music_id: &str, looping: bool) {
        if self.current_music.as_deref() == Some(music_id) {
            return;
        }

        self.stop_music();
        self.current_music = Some(music_id.to_string());

        if self.channels.len() > 15 {
            self.channels[15].playing = Some(PlayingSound {
                sound_id: music_id.to_string(),
                start_time: get_time(),
                looping,
            });
            self.channels[15].volume = self.music_volume;
        }
    }

    pub fn stop_music(&mut self) {
        if self.channels.len() > 15 {
            self.channels[15].playing = None;
        }
        self.current_music = None;
    }

    pub fn stop_channel(&mut self, channel_id: usize) {
        if channel_id < self.channels.len() {
            self.channels[channel_id].playing = None;
        }
    }

    pub fn duck(&mut self, target_volume: f32, duration: f32) {
        self.ducking = DuckingState {
            active: true,
            target_volume,
            current_volume: self.ducking.current_volume,
            duration,
            timer: 0.0,
        };
    }

    pub fn update(&mut self, dt: f32) {
        if self.ducking.active {
            self.ducking.timer += dt;
            let progress = (self.ducking.timer / self.ducking.duration).min(1.0);

            self.ducking.current_volume = self.ducking.current_volume
                + (self.ducking.target_volume - self.ducking.current_volume) * progress;

            if self.ducking.timer >= self.ducking.duration {
                self.ducking.active = false;
                self.ducking.current_volume = self.ducking.target_volume;
            }
        }

        for channel in &mut self.channels {
            if let Some(playing) = &channel.playing {
                if !playing.looping && get_time() - playing.start_time > 5.0 {
                    channel.playing = None;
                }
            }
        }
    }

    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_sfx_volume(&mut self, volume: f32) {
        self.sfx_volume = volume.clamp(0.0, 1.0);
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume.clamp(0.0, 1.0);
    }

    pub fn get_final_volume(&self, channel_id: usize) -> f32 {
        if channel_id >= self.channels.len() {
            return 0.0;
        }

        let channel = &self.channels[channel_id];
        let is_music = channel_id == 15;

        let type_volume = if is_music {
            self.music_volume
        } else {
            self.sfx_volume
        };

        channel.volume * type_volume * self.master_volume * self.ducking.current_volume
    }
}
