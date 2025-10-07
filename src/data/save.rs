use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Clone)]
pub struct SaveData {
    pub profile_name: String,
    pub story_progress: StoryProgress,
    pub unlocks: Unlocks,
    pub statistics: GameStatistics,
    pub settings: GameSettings,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StoryProgress {
    pub current_chapter: u32,
    pub completed_chapters: Vec<u32>,
    pub boss_defeats: Vec<String>,
    pub difficulty: Difficulty,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Unlocks {
    pub characters: Vec<String>,
    pub stages: Vec<String>,
    pub music_tracks: Vec<String>,
    pub gallery_items: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameStatistics {
    pub total_playtime: f64,
    pub matches_played: u32,
    pub matches_won: u32,
    pub highest_combo: u32,
    pub total_damage_dealt: f64,
    pub total_damage_taken: f64,
    pub favorite_character: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub subtitles_enabled: bool,
    pub colorblind_mode: ColorblindMode,
    pub input_buffer_window: f32,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Difficulty {
    Story,
    Normal,
    Hard,
    Extreme,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum ColorblindMode {
    None,
    Protanopia,
    Deuteranopia,
    Tritanopia,
}

pub struct SaveManager {
    save_directory: PathBuf,
    current_save: Option<SaveData>,
    autosave_timer: f32,
    autosave_interval: f32,
}

impl SaveManager {
    pub fn new() -> Self {
        let save_dir = Self::get_save_directory();
        fs::create_dir_all(&save_dir).ok();

        Self {
            save_directory: save_dir,
            current_save: None,
            autosave_timer: 0.0,
            autosave_interval: 60.0,
        }
    }

    fn get_save_directory() -> PathBuf {
        if cfg!(target_os = "windows") {
            PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string()))
                .join("BasVeegArc")
                .join("saves")
        } else if cfg!(target_os = "macos") {
            PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                .join("Library")
                .join("Application Support")
                .join("BasVeegArc")
                .join("saves")
        } else {
            PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                .join(".local")
                .join("share")
                .join("bas-veeg-arc")
                .join("saves")
        }
    }

    pub fn create_new_save(&mut self, profile_name: String) -> SaveData {
        let save = SaveData {
            profile_name,
            story_progress: StoryProgress {
                current_chapter: 0,
                completed_chapters: Vec::new(),
                boss_defeats: Vec::new(),
                difficulty: Difficulty::Normal,
            },
            unlocks: Unlocks {
                characters: vec!["Bas".to_string()],
                stages: vec!["Art Room".to_string()],
                music_tracks: vec!["Main Theme".to_string()],
                gallery_items: Vec::new(),
            },
            statistics: GameStatistics {
                total_playtime: 0.0,
                matches_played: 0,
                matches_won: 0,
                highest_combo: 0,
                total_damage_dealt: 0.0,
                total_damage_taken: 0.0,
                favorite_character: "Bas".to_string(),
            },
            settings: GameSettings {
                master_volume: 1.0,
                sfx_volume: 1.0,
                music_volume: 0.8,
                subtitles_enabled: true,
                colorblind_mode: ColorblindMode::None,
                input_buffer_window: 0.2,
            },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        self.current_save = Some(save.clone());
        save
    }

    pub fn save_to_slot(&self, slot: usize) -> Result<(), String> {
        if let Some(ref save) = self.current_save {
            let filename = format!("save_{}.json", slot);
            let path = self.save_directory.join(filename);

            match serde_json::to_string_pretty(save) {
                Ok(json) => match fs::write(path, json) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Failed to write save file: {}", e)),
                },
                Err(e) => Err(format!("Failed to serialize save data: {}", e)),
            }
        } else {
            Err("No save data to write".to_string())
        }
    }

    pub fn load_from_slot(&mut self, slot: usize) -> Result<SaveData, String> {
        let filename = format!("save_{}.json", slot);
        let path = self.save_directory.join(filename);

        match fs::read_to_string(path) {
            Ok(json) => match serde_json::from_str::<SaveData>(&json) {
                Ok(save) => {
                    self.current_save = Some(save.clone());
                    Ok(save)
                }
                Err(e) => Err(format!("Failed to deserialize save data: {}", e)),
            },
            Err(e) => Err(format!("Failed to read save file: {}", e)),
        }
    }

    pub fn delete_slot(&self, slot: usize) -> Result<(), String> {
        let filename = format!("save_{}.json", slot);
        let path = self.save_directory.join(filename);

        match fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to delete save file: {}", e)),
        }
    }

    pub fn get_save_slots(&self) -> Vec<Option<SaveInfo>> {
        let mut slots = vec![None; 3];

        for i in 0..3 {
            let filename = format!("save_{}.json", i);
            let path = self.save_directory.join(&filename);

            if path.exists() {
                if let Ok(json) = fs::read_to_string(&path) {
                    if let Ok(save) = serde_json::from_str::<SaveData>(&json) {
                        slots[i] = Some(SaveInfo {
                            slot: i,
                            profile_name: save.profile_name,
                            chapter: save.story_progress.current_chapter,
                            playtime: save.statistics.total_playtime,
                            timestamp: save.timestamp,
                        });
                    }
                }
            }
        }

        slots
    }

    pub fn update(&mut self, dt: f32) {
        self.autosave_timer += dt;

        if self.autosave_timer >= self.autosave_interval {
            self.autosave_timer = 0.0;
            self.autosave();
        }
    }

    pub fn autosave(&self) {
        self.save_to_slot(0).ok();
    }

    pub fn get_current_save(&self) -> Option<&SaveData> {
        self.current_save.as_ref()
    }

    pub fn get_current_save_mut(&mut self) -> Option<&mut SaveData> {
        self.current_save.as_mut()
    }
}

#[derive(Clone, Debug)]
pub struct SaveInfo {
    pub slot: usize,
    pub profile_name: String,
    pub chapter: u32,
    pub playtime: f64,
    pub timestamp: u64,
}
