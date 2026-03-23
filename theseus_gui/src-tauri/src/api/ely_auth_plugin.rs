use crate::api::Result;
use tauri::plugin::TauriPlugin;

pub fn init<R: tauri::Runtime>() -> TauriPlugin<R> {
    tauri::plugin::Builder::new("ely-auth")
        .invoke_handler(tauri::generate_handler![
            ely_login,
            ely_logout,
            ely_refresh,
            ely_get_accounts,
            ely_set_default,
            ely_remove_account,
            ely_ensure_authlib,
            ely_check_api_status,
        ])
        .build()
}

#[tauri::command]
pub async fn ely_login(
    username: String,
    password: String,
) -> Result<vio::state::Credentials> {
    Ok(vio::api::minecraft_auth::ely_login(&username, &password).await?)
}

#[tauri::command]
pub async fn ely_logout(user_id: uuid::Uuid) -> Result<()> {
    Ok(vio::api::minecraft_auth::remove_user(user_id).await?)
}

#[tauri::command]
pub async fn ely_refresh(user_id: uuid::Uuid) -> Result<vio::state::Credentials> {
    Ok(vio::api::minecraft_auth::get_user(user_id).await?)
}

#[tauri::command]
pub async fn ely_get_accounts() -> Result<Vec<vio::state::Credentials>> {
    Ok(
        vio::api::minecraft_auth::users()
            .await?
            .into_iter()
            .filter(|account| matches!(account.account_type, vio::state::AccountType::ElyBy { .. }))
            .collect(),
    )
}

#[tauri::command]
pub async fn ely_set_default(user_id: uuid::Uuid) -> Result<()> {
    Ok(vio::api::minecraft_auth::set_default_user(user_id).await?)
}

#[tauri::command]
pub async fn ely_remove_account(user_id: uuid::Uuid) -> Result<()> {
    Ok(vio::api::minecraft_auth::remove_user(user_id).await?)
}

#[tauri::command]
pub async fn ely_ensure_authlib() -> Result<String> {
    match vio::launcher::get_or_download_authlib(None).await {
        Ok(path) => Ok(format!("authlib-injector ready: {}", path.display())),
        Err(e) => Err(crate::api::TheseusSerializableError::Theseus(
            vio::Error::from(vio::ErrorKind::LauncherError(format!(
                "Failed to prepare authlib-injector: {}",
                e
            ))),
        )),
    }
}

#[tauri::command]
pub async fn ely_check_api_status() -> Result<bool> {
    Ok(vio::launcher::check_ely_by_availability().await)
}
