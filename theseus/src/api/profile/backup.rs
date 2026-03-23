//! Резервное копирование профилей - заглушка

use crate::prelude::ProfilePathId;
use serde::{Deserialize, Serialize};

/// Метаданные резервной копии
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub profile_name: String,
    pub profile_uuid: uuid::Uuid,
    pub minecraft_version: String,
    pub modloader: String,
    pub created_at: i64,
    pub total_size: u64,
    pub files_count: usize,
}

/// Экспорт профиля в .zip архив
pub async fn export_profile(
    _profile_path: &ProfilePathId,
    _output_path: &std::path::Path,
) -> crate::Result<BackupMetadata> {
    Err(crate::ErrorKind::InputError("Not implemented".to_string()).into())
}

/// Импорт профиля из .zip архива
pub async fn import_profile(
    _archive_path: &std::path::Path,
    _target_name: Option<String>,
) -> crate::Result<ProfilePathId> {
    Err(crate::ErrorKind::InputError("Not implemented".to_string()).into())
}

pub async fn list_backups(_backup_dir: &std::path::Path) -> crate::Result<Vec<BackupMetadata>> {
    Ok(Vec::new())
}

pub mod emit {
    use crate::event::emit::emit_event;
    use serde_json::json;
    use std::path::Path;
    use uuid::Uuid;

    pub async fn emit_backup_created(
        profile_uuid: &Uuid,
        profile_name: &str,
        backup_path: &Path,
    ) -> crate::Result<()> {
        emit_event("backup:created", json!({ "profile_uuid": profile_uuid, "profile_name": profile_name, "backup_path": backup_path })).await
    }

    pub async fn emit_backup_imported(
        profile_uuid: &Uuid,
        profile_name: &str,
        backup_path: &Path,
    ) -> crate::Result<()> {
        emit_event("backup:imported", json!({ "profile_uuid": profile_uuid, "profile_name": profile_name, "backup_path": backup_path })).await
    }
}
