//! Детектор конфликтов модов - заглушка

use crate::prelude::ProfilePathId;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictReport {
    pub profile_uuid: Uuid,
    pub profile_name: String,
    pub conflicts: Vec<Conflict>,
    pub warnings: Vec<Warning>,
    pub is_safe: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conflict {
    pub severity: ConflictSeverity,
    pub mod_a: ModInfo,
    pub mod_b: ModInfo,
    pub reason: String,
    pub solution: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warning {
    pub severity: WarningSeverity,
    pub mod_info: ModInfo,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModInfo {
    pub name: String,
    pub id: String,
    pub version: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConflictSeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum WarningSeverity {
    High,
    Medium,
    Low,
    Info,
}

pub async fn check_conflicts(profile_path: &ProfilePathId) -> crate::Result<ConflictReport> {
    Ok(ConflictReport {
        profile_uuid: Uuid::nil(),
        profile_name: String::new(),
        conflicts: Vec::new(),
        warnings: Vec::new(),
        is_safe: true,
    })
}

pub async fn resolve_conflicts(_profile_path: &ProfilePathId, _conflicts: &[Conflict]) -> crate::Result<usize> {
    Ok(0)
}

pub mod emit {
    use crate::event::emit::emit_event;
    use serde_json::json;
    use uuid::Uuid;

    pub async fn emit_conflicts_detected(_profile_uuid: &Uuid, _conflicts_count: usize) -> crate::Result<()> {
        emit_event("conflicts:detected", json!({})).await
    }

    pub async fn emit_conflicts_resolved(_profile_uuid: &Uuid, _removed_count: usize) -> crate::Result<()> {
        emit_event("conflicts:resolved", json!({})).await
    }
}
