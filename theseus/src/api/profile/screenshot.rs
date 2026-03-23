//! Менеджер скриншотов - заглушка

use crate::prelude::ProfilePathId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotInfo {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub created_at: i64,
    pub width: u32,
    pub height: u32,
    pub thumbnail: Option<Vec<u8>>,
}

pub async fn list_screenshots(profile_path: &ProfilePathId) -> crate::Result<Vec<ScreenshotInfo>> {
    // Возвращаем пустой список - реализация требует доступа к профилю
    Ok(Vec::new())
}

pub async fn delete_screenshot(profile_path: &ProfilePathId, _screenshot_name: &str) -> crate::Result<()> {
    emit::emit_screenshot_deleted(&uuid::Uuid::nil(), "stub").await?;
    Ok(())
}

pub async fn open_screenshot(profile_path: &ProfilePathId, screenshot_name: &str) -> crate::Result<()> {
    open::that(format!("{:?}/{}", profile_path, screenshot_name))?;
    Ok(())
}

pub async fn take_screenshot(profile_path: &ProfilePathId) -> crate::Result<PathBuf> {
    Ok(PathBuf::from(format!("{:?}/screenshots", profile_path)))
}

pub async fn get_screenshot_data(_profile_path: &ProfilePathId, _screenshot_name: &str) -> crate::Result<Vec<u8>> {
    Ok(Vec::new())
}

pub mod emit {
    use crate::event::emit::emit_event;
    use serde_json::json;
    use std::path::Path;
    use uuid::Uuid;

    pub async fn emit_screenshot_deleted(_profile_uuid: &Uuid, _screenshot_name: &str) -> crate::Result<()> {
        emit_event("screenshot:deleted", json!({})).await
    }

    pub async fn emit_screenshot_taken(_profile_uuid: &Uuid, _filename: &str, _path: &Path) -> crate::Result<()> {
        emit_event("screenshot:taken", json!({})).await
    }
}
