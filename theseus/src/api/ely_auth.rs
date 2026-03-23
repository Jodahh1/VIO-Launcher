//! Ely.by Authentication API
//! 
//! Implements Yggdrasil-compatible authentication protocol
//! https://docs.ely.by/en/minecraft-auth.html

use crate::util::fetch::REQWEST_CLIENT;
use serde::{Deserialize, Serialize};

const ELY_BASE_URL: &str = "https://authserver.ely.by";

/// Запрос аутентификации
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElyByAuthRequest {
    pub username: String,
    pub password: String,
    pub client_token: String,
    pub request_user: bool,
}

/// Ответ аутентификации
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ElyByAuthResponse {
    pub access_token: String,
    pub client_token: String,
    #[serde(default)]
    pub available_profiles: Vec<Profile>,
    pub selected_profile: Profile,
    pub user: Option<User>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Profile {
    pub id: String,      // UUID без дефисов
    pub name: String,    // Username
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(default)]
    pub properties: Vec<UserProperty>,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserProperty {
    pub name: String,
    pub value: String,
}

/// Запрос обновления токена
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElyByRefreshRequest {
    pub access_token: String,
    pub client_token: String,
    pub request_user: bool,
}

/// Ответ обновления
pub type ElyByRefreshResponse = ElyByAuthResponse;

/// Запрос валидации
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElyByValidateRequest {
    pub access_token: String,
}

/// Ошибка аутентификации
#[derive(Debug, Deserialize)]
pub struct ElyByError {
    pub error: String,
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(default)]
    pub cause: Option<String>,
}

impl std::fmt::Display for ElyByError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error, self.error_message)
    }
}

/// Основная аутентификация
#[tracing::instrument]
pub async fn authenticate(
    username: &str,
    password: &str,
    client_token: &str,
) -> crate::Result<ElyByAuthResponse> {
    let request = ElyByAuthRequest {
        username: username.to_string(),
        password: password.to_string(),
        client_token: client_token.to_string(),
        request_user: true,
    };

    let url = format!("{}/auth/authenticate", ELY_BASE_URL);
    
    let response = REQWEST_CLIENT
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            crate::ErrorKind::LauncherError(format!(
                "Network error during authentication: {}",
                e
            ))
        })?;

    // Обработка ошибок
    if !response.status().is_success() {
        let error: ElyByError = response.json().await.map_err(|e| {
            crate::ErrorKind::OtherError(format!(
                "Failed to parse error response: {}",
                e
            ))
        })?;
        
        return Err(handle_auth_error(&error).into());
    }

    response.json().await.map_err(|e| {
        crate::ErrorKind::OtherError(format!(
            "Failed to parse auth response: {}",
            e
        )).into()
    })
}

/// Обновление токена
#[tracing::instrument]
pub async fn refresh(
    access_token: &str,
    client_token: &str,
) -> crate::Result<ElyByRefreshResponse> {
    let request = ElyByRefreshRequest {
        access_token: access_token.to_string(),
        client_token: client_token.to_string(),
        request_user: true,
    };

    let url = format!("{}/auth/refresh", ELY_BASE_URL);
    
    let response = REQWEST_CLIENT
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            crate::ErrorKind::LauncherError(format!(
                "Network error during token refresh: {}",
                e
            ))
        })?;

    if !response.status().is_success() {
        let error: ElyByError = response.json().await?;
        return Err(handle_auth_error(&error).into());
    }

    response.json().await.map_err(|e| {
        crate::ErrorKind::OtherError(format!(
            "Failed to parse refresh response: {}",
            e
        )).into()
    })
}

/// Валидация токена
#[tracing::instrument]
pub async fn validate(access_token: &str) -> crate::Result<bool> {
    let request = ElyByValidateRequest {
        access_token: access_token.to_string(),
    };

    let url = format!("{}/auth/validate", ELY_BASE_URL);
    
    let response = REQWEST_CLIENT
        .post(&url)
        .json(&request)
        .send()
        .await
        .map_err(|e| {
            crate::ErrorKind::LauncherError(format!(
                "Network error during validation: {}",
                e
            ))
        })?;

    // 200 = валиден, 401 = невалиден
    Ok(response.status().is_success())
}

/// Обработка ошибок аутентификации
fn handle_auth_error(error: &ElyByError) -> crate::Error {
    match error.error.as_str() {
        "ForbiddenOperationException" => {
            if error.error_message.contains("Account protected with two factor auth.") {
                crate::ErrorKind::OtherError(
                    "ELY_2FA_REQUIRED: Account protected with two factor auth.".into(),
                )
                .into()
            } else if error.error_message.contains("Invalid credentials") {
                crate::ErrorKind::OtherError(format!(
                    "ELY_INVALID_CREDENTIALS: {}",
                    error.error_message
                ))
                .into()
            } else {
                crate::ErrorKind::OtherError(format!(
                    "ELY_AUTH_ERROR: {}",
                    error.error_message
                ))
                .into()
            }
        }
        "IllegalArgumentException" => {
            crate::ErrorKind::OtherError(
                "Invalid request parameters".into()
            ).into()
        }
        _ => {
            crate::ErrorKind::OtherError(format!(
                "ELY_AUTH_ERROR: {}",
                error.error_message
            )).into()
        }
    }
}
