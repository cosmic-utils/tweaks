use std::path::PathBuf;

/// Deletes `~/.config/cosmic/<app_id>/v1/` so COSMIC recreates it from
/// `/usr/share/cosmic/` defaults on next launch.
pub fn reset_cosmic_config(app_id: &str) {
    let config_dir: Option<PathBuf> =
        dirs::home_dir().map(|home| home.join(".config").join("cosmic").join(app_id).join("v1"));

    if let Some(dir) = config_dir {
        if dir.exists() {
            if let Err(e) = std::fs::remove_dir_all(&dir) {
                log::error!("Failed to reset config for {}: {}", app_id, e);
            }
        }
    }
}
