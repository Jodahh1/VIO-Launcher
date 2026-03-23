//! Управление аккаунтами - заглушка

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub uuid: Uuid,
    pub username: String,
    pub is_online: bool,
    pub is_default: bool,
    pub last_played: Option<i64>,
    pub skin_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchResult {
    pub previous_account: Option<Uuid>,
    pub current_account: Option<Uuid>,
    pub success: bool,
}

pub async fn get_all_accounts() -> crate::Result<Vec<AccountInfo>> {
    Ok(Vec::new())
}

pub async fn get_current_account() -> crate::Result<Option<AccountInfo>> {
    Ok(None)
}

pub async fn switch_account(target_uuid: Uuid) -> crate::Result<SwitchResult> {
    Ok(SwitchResult {
        previous_account: None,
        current_account: Some(target_uuid),
        success: true,
    })
}

pub async fn set_default_account(_uuid: Uuid) -> crate::Result<()> {
    Ok(())
}

pub async fn add_offline_account(username: String) -> crate::Result<Uuid> {
    let uuid = Uuid::new_v4();
    emit::emit_account_added(uuid, &username, false).await?;
    Ok(uuid)
}

pub async fn remove_account(_uuid: Uuid) -> crate::Result<()> {
    Ok(())
}

pub async fn refresh_account_info(uuid: Uuid) -> crate::Result<AccountInfo> {
    Ok(AccountInfo {
        uuid,
        username: String::new(),
        is_online: false,
        is_default: false,
        last_played: None,
        skin_url: None,
    })
}

pub mod emit {
    use crate::event::emit::emit_event;
    use serde_json::json;
    use uuid::Uuid;

    pub async fn emit_account_switched(_previous: Option<Uuid>, _current: Option<Uuid>) -> crate::Result<()> {
        emit_event("account:switched", json!({})).await
    }

    pub async fn emit_default_account_set(_uuid: Uuid) -> crate::Result<()> {
        emit_event("account:default_set", json!({})).await
    }

    pub async fn emit_account_added(_uuid: Uuid, _username: &str, _is_online: bool) -> crate::Result<()> {
        emit_event("account:added", json!({})).await
    }

    pub async fn emit_account_removed(_uuid: Uuid) -> crate::Result<()> {
        emit_event("account:removed", json!({})).await
    }
}
