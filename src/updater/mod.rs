use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_REPO: &str = "compiledkernel-idk/bas-veeg-arc";
const UPDATE_CHECK_URL: &str = "https://api.github.com/repos/compiledkernel-idk/bas-veeg-arc/releases/latest";

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub available: bool,
    pub latest_version: String,
    pub download_url: Option<String>,
    pub changelog: String,
}

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    name: String,
    body: String,
    assets: Vec<GithubAsset>,
}

#[derive(Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

pub struct Updater {
    pub status: UpdateStatus,
    pub info: Option<UpdateInfo>,
    pub download_progress: f32,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateStatus {
    Idle,
    Checking,
    UpdateAvailable,
    Downloading,
    ReadyToInstall,
    Installing,
    Error,
    UpToDate,
}

impl Updater {
    pub fn new() -> Self {
        Self {
            status: UpdateStatus::Idle,
            info: None,
            download_progress: 0.0,
            error_message: None,
        }
    }

    pub fn check_for_updates(&mut self) {
        self.status = UpdateStatus::Checking;
        self.error_message = None;

        match self.fetch_latest_release() {
            Ok(info) => {
                if info.available {
                    self.status = UpdateStatus::UpdateAvailable;
                    self.info = Some(info);
                } else {
                    self.status = UpdateStatus::UpToDate;
                }
            }
            Err(e) => {
                self.status = UpdateStatus::Error;
                self.error_message = Some(format!("Failed to check for updates: {}", e));
            }
        }
    }

    fn fetch_latest_release(&self) -> Result<UpdateInfo, String> {
        // Fetch from GitHub API with user agent (required by GitHub)
        let response = minreq::get(UPDATE_CHECK_URL)
            .with_header("User-Agent", "bas-veeg-arc-updater")
            .send()
            .map_err(|e| format!("Network error: {}", e))?;

        if response.status_code != 200 {
            return Err(format!("GitHub API returned status {}", response.status_code));
        }

        let release: GithubRelease = serde_json::from_str(response.as_str().map_err(|e| e.to_string())?)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        // Remove 'v' prefix if present
        let latest_version = release.tag_name.trim_start_matches('v').to_string();
        let current_version = CURRENT_VERSION.trim_start_matches('v');

        let available = Self::is_newer_version(&latest_version, current_version);

        // Find the appropriate download URL for this platform
        let download_url = if available {
            self.find_platform_asset(&release.assets)
        } else {
            None
        };

        Ok(UpdateInfo {
            available,
            latest_version: latest_version.clone(),
            download_url,
            changelog: release.body,
        })
    }

    fn find_platform_asset(&self, assets: &[GithubAsset]) -> Option<String> {
        #[cfg(target_os = "windows")]
        let platform_name = "windows";

        #[cfg(target_os = "linux")]
        let platform_name = "linux";

        #[cfg(target_os = "macos")]
        let platform_name = "macos";

        // Look for asset containing platform name
        for asset in assets {
            let name_lower = asset.name.to_lowercase();
            if name_lower.contains(platform_name) {
                #[cfg(target_os = "windows")]
                if name_lower.ends_with(".exe") {
                    return Some(asset.browser_download_url.clone());
                }

                #[cfg(not(target_os = "windows"))]
                if !name_lower.ends_with(".exe") {
                    return Some(asset.browser_download_url.clone());
                }
            }
        }

        None
    }

    fn is_newer_version(latest: &str, current: &str) -> bool {
        // Simple version comparison (assumes semantic versioning)
        let parse_version = |v: &str| -> Vec<u32> {
            v.split('.')
                .filter_map(|s| s.parse().ok())
                .collect()
        };

        let latest_parts = parse_version(latest);
        let current_parts = parse_version(current);

        for i in 0..latest_parts.len().max(current_parts.len()) {
            let l = latest_parts.get(i).unwrap_or(&0);
            let c = current_parts.get(i).unwrap_or(&0);

            if l > c {
                return true;
            } else if l < c {
                return false;
            }
        }

        false
    }

    pub fn download_and_install(&mut self) {
        // Clone the URL to avoid borrow checker issues
        let url = if let Some(info) = &self.info {
            info.download_url.clone()
        } else {
            None
        };

        if let Some(url) = url {
            self.status = UpdateStatus::Downloading;

            match self.download_update(&url) {
                Ok(temp_path) => {
                    self.status = UpdateStatus::Installing;
                    match self.install_update(&temp_path) {
                        Ok(_) => {
                            self.status = UpdateStatus::ReadyToInstall;
                        }
                        Err(e) => {
                            self.status = UpdateStatus::Error;
                            self.error_message = Some(format!("Installation failed: {}", e));
                        }
                    }
                }
                Err(e) => {
                    self.status = UpdateStatus::Error;
                    self.error_message = Some(format!("Download failed: {}", e));
                }
            }
        }
    }

    fn download_update(&mut self, url: &str) -> Result<PathBuf, String> {
        let response = minreq::get(url)
            .with_header("User-Agent", "bas-veeg-arc-updater")
            .send()
            .map_err(|e| format!("Download error: {}", e))?;

        if response.status_code != 200 {
            return Err(format!("Download failed with status {}", response.status_code));
        }

        // Save to temp file
        let temp_dir = std::env::temp_dir();

        #[cfg(target_os = "windows")]
        let temp_file = temp_dir.join("bas-veeg-arc-update.exe");

        #[cfg(not(target_os = "windows"))]
        let temp_file = temp_dir.join("bas-veeg-arc-update");

        fs::write(&temp_file, response.as_bytes())
            .map_err(|e| format!("Failed to save update: {}", e))?;

        self.download_progress = 100.0;

        Ok(temp_file)
    }

    fn install_update(&self, temp_path: &PathBuf) -> Result<(), String> {
        let current_exe = std::env::current_exe()
            .map_err(|e| format!("Failed to get current executable path: {}", e))?;

        #[cfg(target_os = "windows")]
        {
            // On Windows: rename old exe, move new one in place
            let backup_path = current_exe.with_extension("exe.old");

            // Remove old backup if exists
            let _ = fs::remove_file(&backup_path);

            // Rename current exe to .old
            fs::rename(&current_exe, &backup_path)
                .map_err(|e| format!("Failed to backup current executable: {}", e))?;

            // Move new exe to current location
            fs::copy(temp_path, &current_exe)
                .map_err(|e| format!("Failed to install new executable: {}", e))?;

            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            // On Linux/Unix: replace binary directly
            use std::os::unix::fs::PermissionsExt;

            // Make the new file executable
            let mut perms = fs::metadata(temp_path)
                .map_err(|e| format!("Failed to get file metadata: {}", e))?
                .permissions();
            perms.set_mode(0o755);
            fs::set_permissions(temp_path, perms)
                .map_err(|e| format!("Failed to set permissions: {}", e))?;

            // Backup current
            let backup_path = format!("{}.old", current_exe.display());
            let _ = fs::remove_file(&backup_path);
            fs::copy(&current_exe, &backup_path)
                .map_err(|e| format!("Failed to backup: {}", e))?;

            // Replace with new version
            fs::copy(temp_path, &current_exe)
                .map_err(|e| format!("Failed to install update: {}", e))?;

            Ok(())
        }
    }

    pub fn restart_game(&self) {
        let current_exe = std::env::current_exe().unwrap();

        #[cfg(target_os = "windows")]
        {
            std::process::Command::new(current_exe)
                .spawn()
                .ok();
        }

        #[cfg(not(target_os = "windows"))]
        {
            std::process::Command::new(current_exe)
                .spawn()
                .ok();
        }

        std::process::exit(0);
    }
}
