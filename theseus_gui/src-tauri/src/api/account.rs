//! Tauri commands для управления аккаунтами

use crate::api::Result;
use vio::api::{account_manager::*, AccountInfo, SwitchResult};
use uuid::Uuid;

pub fn init<R: tauri::Runtime>() -> tauri::plugin::TauriPlugin<R> {
    tauri::plugin::Builder::new("account")
        .invoke_handler(tauri::generate_handler![
            account_get_all,
            account_get_current,
            account_switch,
            account_set_default,
            account_add_offline,
            account_remove,
            account_refresh_info,
        ])
        .build()
}

/// Получить все аккаунты
#[tauri::command]
pub async fn account_get_all() -> Result<Vec<AccountInfo>> {
    let accounts = get_all_accounts().await?;
    Ok(accounts)
}

/// Получить текущий аккаунт
#[tauri::command]
pub async fn account_get_current() -> Result<Option<AccountInfo>> {
    let account = get_current_account().await?;
    Ok(account)
}

/// Переключиться на аккаунт
#[tauri::command]
pub async fn account_switch(
    target_uuid: Uuid,
) -> Result<SwitchResult> {
    let result = switch_account(target_uuid).await?;
    Ok(result)
}

/// Установить аккаунт по умолчанию
#[tauri::command]
pub async fn account_set_default(
    uuid: Uuid,
) -> Result<()> {
    set_default_account(uuid).await?;
    Ok(())
}

/// Добавить офлайн аккаунт
#[tauri::command]
pub async fn account_add_offline(
    username: String,
) -> Result<Uuid> {
    let uuid = add_offline_account(username).await?;
    Ok(uuid)
}

/// Удалить аккаунт
#[tauri::command]
pub async fn account_remove(
    uuid: Uuid,
) -> Result<()> {
    remove_account(uuid).await?;
    Ok(())
}

/// Обновить информацию об аккаунте
#[tauri::command]
pub async fn account_refresh_info(
    uuid: Uuid,
) -> Result<AccountInfo> {
    let info = refresh_account_info(uuid).await?;
    Ok(info)
}
