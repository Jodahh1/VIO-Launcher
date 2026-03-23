//! Авто-обновление модов - заглушка

use crate::prelude::ProfilePathId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoUpdateSettings {
    pub enabled: bool,
    pub check_interval_hours: u64,
    pub auto_download: bool,
    pub auto_install: bool,
    pub notify_only: bool,
    pub exclude_mods: Vec<String>,
    pub last_check: Option<i64>,
}

impl Default for AutoUpdateSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            check_interval_hours: 24,
            auto_download: true,
            auto_install: false,
            notify_only: false,
            exclude_mods: Vec::new(),
            last_check: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckResult {
    pub profile_uuid: uuid::Uuid,
    pub profile_name: String,
    pub updates_available: Vec<ModUpdate>,
    pub updated_count: usize,
    pub failed_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModUpdate {
    pub mod_id: String,
    pub mod_name: String,
    pub current_version: String,
    pub new_version: String,
    pub download_url: Option<String>,
    pub is_critical: bool,
}

pub async fn check_for_updates(_profile_path: &ProfilePathId) -> crate::Result<UpdateCheckResult> {
    Ok(UpdateCheckResult {
        profile_uuid: uuid::Uuid::nil(),
        profile_name: String::new(),
        updates_available: Vec::new(),
        updated_count: 0,
        failed_count: 0,
    })
}

pub async fn install_all_updates(_profile_path: &ProfilePathId) -> crate::Result<UpdateCheckResult> {
    Ok(UpdateCheckResult {
        profile_uuid: uuid::Uuid::nil(),
        profile_name: String::new(),
        updates_available: Vec::new(),
        updated_count: 0,
        failed_count: 0,
    })
}

pub async fn get_auto_update_settings(_profile_path: &ProfilePathId) -> crate::Result<AutoUpdateSettings> {
    Ok(AutoUpdateSettings::default())
}

pub async fn set_auto_update_settings(_profile_path: &ProfilePathId, _settings: AutoUpdateSettings) -> crate::Result<()> {
    Ok(())
}

pub async fn should_check_updates(_profile_path: &ProfilePathId) -> crate::Result<bool> {
    Ok(false)
}

pub async fn start_background_update_checker() -> crate::Result<()> {
    Ok(())
}

pub mod emit {
    use crate::event::emit::emit_event;
    use serde_json::json;
    use uuid::Uuid;

    pub async fn emit_updates_available(_profile_uuid: &Uuid, _updates_count: usize) -> crate::Result<()> {
        emit_event("update:available", json!({})).await
    }

    pub async fn emit_mod_updated(_profile_uuid: &Uuid, _mod_id: &str, _version: &str) -> crate::Result<()> {
        emit_event("update:mod", json!({})).await
    }

    pub async fn emit_settings_changed(_profile_uuid: &Uuid) -> crate::Result<()> {
        emit_event("update:settings_changed", json!({})).await
    }
}
