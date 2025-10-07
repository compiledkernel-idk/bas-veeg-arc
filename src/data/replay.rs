use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Clone)]
pub struct Replay {
    pub metadata: ReplayMetadata,
    pub frames: Vec<ReplayFrame>,
    pub checksum: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReplayMetadata {
    pub version: String,
    pub timestamp: u64,
    pub duration: f32,
    pub stage: String,
    pub characters: Vec<String>,
    pub winner: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReplayFrame {
    pub frame_number: u32,
    pub inputs: Vec<InputSnapshot>,
    pub positions: Vec<PositionSnapshot>,
    pub health_values: Vec<f32>,
    pub meter_values: Vec<f32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InputSnapshot {
    pub player_id: u8,
    pub buttons: u32,
    pub stick_x: f32,
    pub stick_y: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PositionSnapshot {
    pub entity_id: u32,
    pub x: f32,
    pub y: f32,
}

pub struct ReplayManager {
    recording: bool,
    playing: bool,
    current_replay: Option<Replay>,
    frame_buffer: VecDeque<ReplayFrame>,
    playback_frame: usize,
    recording_frame: u32,
    max_frames: usize,
}

impl ReplayManager {
    pub fn new() -> Self {
        Self {
            recording: false,
            playing: false,
            current_replay: None,
            frame_buffer: VecDeque::new(),
            playback_frame: 0,
            recording_frame: 0,
            max_frames: 36000,
        }
    }

    pub fn start_recording(&mut self, stage: String, characters: Vec<String>) {
        self.recording = true;
        self.playing = false;
        self.recording_frame = 0;
        self.frame_buffer.clear();

        let metadata = ReplayMetadata {
            version: "1.0.0".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            duration: 0.0,
            stage,
            characters,
            winner: String::new(),
        };

        self.current_replay = Some(Replay {
            metadata,
            frames: Vec::new(),
            checksum: 0,
        });
    }

    pub fn stop_recording(&mut self, winner: String) -> Option<Replay> {
        if !self.recording {
            return None;
        }

        self.recording = false;

        if let Some(ref mut replay) = self.current_replay {
            replay.metadata.winner = winner;
            replay.metadata.duration = self.recording_frame as f32 / 60.0;
            replay.frames = self.frame_buffer.drain(..).collect();
            let checksum = Self::calculate_checksum(&replay.frames);
            replay.checksum = checksum;
            Some(replay.clone())
        } else {
            None
        }
    }

    pub fn start_playback(&mut self, replay: Replay) {
        self.playing = true;
        self.recording = false;
        self.playback_frame = 0;
        self.current_replay = Some(replay);
    }

    pub fn stop_playback(&mut self) {
        self.playing = false;
        self.playback_frame = 0;
    }

    pub fn record_frame(&mut self, frame: ReplayFrame) {
        if !self.recording {
            return;
        }

        self.frame_buffer.push_back(frame);
        self.recording_frame += 1;

        if self.frame_buffer.len() > self.max_frames {
            self.frame_buffer.pop_front();
        }
    }

    pub fn get_playback_frame(&mut self) -> Option<ReplayFrame> {
        if !self.playing {
            return None;
        }

        if let Some(ref replay) = self.current_replay {
            if self.playback_frame < replay.frames.len() {
                let frame = replay.frames[self.playback_frame].clone();
                self.playback_frame += 1;
                Some(frame)
            } else {
                self.stop_playback();
                None
            }
        } else {
            None
        }
    }

    pub fn seek(&mut self, frame: usize) {
        if self.playing {
            if let Some(ref replay) = self.current_replay {
                self.playback_frame = frame.min(replay.frames.len() - 1);
            }
        }
    }

    pub fn get_playback_progress(&self) -> f32 {
        if !self.playing {
            return 0.0;
        }

        if let Some(ref replay) = self.current_replay {
            if !replay.frames.is_empty() {
                self.playback_frame as f32 / replay.frames.len() as f32
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    fn calculate_checksum(frames: &[ReplayFrame]) -> u32 {
        let mut checksum: u32 = 0;

        for frame in frames {
            checksum = checksum.wrapping_add(frame.frame_number);

            for input in &frame.inputs {
                checksum = checksum.wrapping_add(input.player_id as u32);
                checksum = checksum.wrapping_add(input.buttons);
            }

            for pos in &frame.positions {
                checksum = checksum.wrapping_add(pos.entity_id);
                checksum = checksum.wrapping_add(pos.x as u32);
                checksum = checksum.wrapping_add(pos.y as u32);
            }
        }

        checksum
    }

    pub fn verify_replay(&self, replay: &Replay) -> bool {
        let calculated = Self::calculate_checksum(&replay.frames);
        calculated == replay.checksum
    }

    pub fn is_recording(&self) -> bool {
        self.recording
    }

    pub fn is_playing(&self) -> bool {
        self.playing
    }
}
