use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpgradeId {
    AttackBoost,
    HealthBoost,
    SpeedBoost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopData {
    #[serde(default)]
    pub currency: u32,
    #[serde(default)]
    pub owned_upgrades: Vec<UpgradeId>,
}

impl Default for ShopData {
    fn default() -> Self {
        Self {
            currency: 0,
            owned_upgrades: Vec::new(),
        }
    }
}

pub struct ShopManager {
    data: ShopData,
    path: PathBuf,
}

impl ShopManager {
    pub fn load() -> Self {
        let path = Self::shop_file_path();

        if let Some(dir) = path.parent() {
            let _ = fs::create_dir_all(dir);
        }

        let data = if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|json| serde_json::from_str(&json).ok())
                .unwrap_or_default()
        } else {
            ShopData::default()
        };

        Self { data, path }
    }

    fn shop_file_path() -> PathBuf {
        let base = if cfg!(target_os = "windows") {
            PathBuf::from(std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string()))
                .join("BasVeegArc")
        } else if cfg!(target_os = "macos") {
            PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                .join("Library")
                .join("Application Support")
                .join("BasVeegArc")
        } else {
            PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
                .join(".local")
                .join("share")
                .join("bas-veeg-arc")
        };

        base.join("shop").join("shop_data.json")
    }

    pub fn currency(&self) -> u32 {
        self.data.currency
    }

    pub fn add_currency(&mut self, amount: u32) {
        self.data.currency = self.data.currency.saturating_add(amount);
    }

    pub fn try_purchase(&mut self, upgrade: UpgradeId, cost: u32) -> bool {
        if self.has_upgrade(upgrade) {
            return false;
        }

        if self.data.currency < cost {
            return false;
        }

        self.data.currency -= cost;
        self.data.owned_upgrades.push(upgrade);
        true
    }

    pub fn has_upgrade(&self, upgrade: UpgradeId) -> bool {
        self.data
            .owned_upgrades
            .iter()
            .copied()
            .any(|u| u == upgrade)
    }

    pub fn owned_upgrades(&self) -> &[UpgradeId] {
        &self.data.owned_upgrades
    }

    pub fn save(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.data)
            .map_err(|e| format!("Failed to serialize shop data: {}", e))?;

        fs::write(&self.path, json).map_err(|e| format!("Failed to write shop data: {}", e))
    }
}
