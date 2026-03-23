//! Офлайн скины - заглушка

use crate::prelude::ProfilePathId;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinInfo {
    pub player_name: String,
    pub skin_path: PathBuf,
    pub skin_type: SkinType,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SkinType {
    #[default]
    Steve,
    Slim,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OfflineSkinsConfig {
    pub enabled: bool,
    pub skins: HashMap<String, SkinInfo>,
    pub default_skin: Option<String>,
}

pub async fn set_offline_skin(
    _profile_path: &ProfilePathId,
    player_name: &str,
    skin_path: &Path,
    skin_type: SkinType,
) -> crate::Result<SkinInfo> {
    Ok(SkinInfo {
        player_name: player_name.to_string(),
        skin_path: skin_path.to_path_buf(),
        skin_type,
        is_default: false,
    })
}

pub async fn remove_offline_skin(_profile_path: &ProfilePathId, _player_name: &str) -> crate::Result<()> {
    Ok(())
}

pub async fn get_skins(_profile_path: &ProfilePathId) -> crate::Result<Vec<SkinInfo>> {
    Ok(Vec::new())
}

pub async fn set_default_skin(_profile_path: &ProfilePathId, _player_name: &str) -> crate::Result<()> {
    Ok(())
}

pub async fn download_skin(
    _profile_path: &ProfilePathId,
    player_name: &str,
    _skin_url: &str,
    skin_type: SkinType,
) -> crate::Result<SkinInfo> {
    Ok(SkinInfo {
        player_name: player_name.to_string(),
        skin_path: PathBuf::new(),
        skin_type,
        is_default: false,
    })
}

pub async fn apply_skin(_profile_path: &ProfilePathId, _player_name: &str) -> crate::Result<()> {
    Ok(())
}

pub async fn get_default_skin(_profile_path: &ProfilePathId) -> crate::Result<Option<SkinInfo>> {
    Ok(None)
}

pub async fn toggle_offline_skins(_profile_path: &ProfilePathId, _enabled: bool) -> crate::Result<()> {
    Ok(())
}

pub mod emit {
    use crate::event::emit::emit_event;
    use serde_json::json;
    use std::path::Path;
    use uuid::Uuid;

    pub async fn emit_skin_set(_profile_uuid: &Uuid, _player_name: &str, _skin_path: &Path) -> crate::Result<()> {
        emit_event("skin:set", json!({})).await
    }

    pub async fn emit_skin_removed(_profile_uuid: &Uuid, _player_name: &str) -> crate::Result<()> {
        emit_event("skin:removed", json!({})).await
    }

    pub async fn emit_default_skin_set(_profile_uuid: &Uuid, _player_name: &str) -> crate::Result<()> {
        emit_event("skin:default_set", json!({})).await
    }

    pub async fn emit_skin_applied(_profile_uuid: &Uuid, _player_name: &str, _target_path: &Path) -> crate::Result<()> {
        emit_event("skin:applied", json!({})).await
    }

    pub async fn emit_skins_toggled(_profile_uuid: &Uuid, _enabled: bool) -> crate::Result<()> {
        emit_event("skin:toggled", json!({})).await
    }
}
