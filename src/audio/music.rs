pub struct MusicManager {
    current_track: Option<String>,
    queued_track: Option<String>,
    crossfade_duration: f32,
    crossfade_timer: f32,
    stem_volumes: StemVolumes,
}

#[derive(Clone, Debug)]
pub struct StemVolumes {
    pub drums: f32,
    pub bass: f32,
    pub melody: f32,
    pub harmony: f32,
}

impl MusicManager {
    pub fn new() -> Self {
        Self {
            current_track: None,
            queued_track: None,
            crossfade_duration: 2.0,
            crossfade_timer: 0.0,
            stem_volumes: StemVolumes {
                drums: 1.0,
                bass: 1.0,
                melody: 1.0,
                harmony: 1.0,
            },
        }
    }

    pub fn play_track(&mut self, track_name: String) {
        if self.current_track.is_some() {
            self.queued_track = Some(track_name);
            self.crossfade_timer = 0.0;
        } else {
            self.current_track = Some(track_name);
        }
    }

    pub fn update(&mut self, dt: f32) {
        if self.queued_track.is_some() {
            self.crossfade_timer += dt;

            if self.crossfade_timer >= self.crossfade_duration {
                self.current_track = self.queued_track.take();
                self.crossfade_timer = 0.0;
            }
        }
    }

    pub fn set_stem_volume(&mut self, stem: MusicStem, volume: f32) {
        match stem {
            MusicStem::Drums => self.stem_volumes.drums = volume.clamp(0.0, 1.0),
            MusicStem::Bass => self.stem_volumes.bass = volume.clamp(0.0, 1.0),
            MusicStem::Melody => self.stem_volumes.melody = volume.clamp(0.0, 1.0),
            MusicStem::Harmony => self.stem_volumes.harmony = volume.clamp(0.0, 1.0),
        }
    }

    pub fn get_crossfade_volumes(&self) -> (f32, f32) {
        if self.queued_track.is_some() && self.crossfade_timer > 0.0 {
            let progress = (self.crossfade_timer / self.crossfade_duration).clamp(0.0, 1.0);
            (1.0 - progress, progress)
        } else {
            (1.0, 0.0)
        }
    }
}

#[derive(Clone, Debug)]
pub enum MusicStem {
    Drums,
    Bass,
    Melody,
    Harmony,
}
