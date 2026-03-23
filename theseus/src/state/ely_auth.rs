//! Ely.by Authentication Types
//! 
//! Data structures for Ely.by account management

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Тип аккаунта для поддержки множественных провайдеров
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum AccountType {
    /// Microsoft/XBOX аккаунт (оригинальный Theseus)
    Microsoft,
    /// Оффлайн аккаунт (пиратский)
    Offline,
    /// Ely.by аккаунт
    ElyBy {
        /// ID профиля (UUID без дефисов)
        profile_id: String,
    },
}

impl Default for AccountType {
    fn default() -> Self {
        AccountType::Microsoft
    }
}

/// Дополнительные данные Ely.by
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ElyByData {
    /// UUID пользователя в формате без дефисов
    pub uuid: String,
    /// Email пользователя (опционально)
    pub email: Option<String>,
    /// URL скина
    pub skin_url: Option<String>,
    /// URL плаща (cape)
    pub cape_url: Option<String>,
    /// Модель скина (slim/steve)
    pub skin_model: Option<String>,
    /// Signature для текстур
    pub texture_signature: Option<String>,
    /// Texture value (base64)
    pub texture_value: Option<String>,
}

/// Расширенные Credentials с поддержкой Ely.by
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElyByCredentials {
    /// Уникальный ID аккаунта в лаунчере
    pub id: Uuid,
    /// Тип аккаунта
    #[serde(default)]
    pub account_type: AccountType,
    /// Имя пользователя (никнейм)
    pub username: String,
    /// Access token для Minecraft сессии
    pub access_token: String,
    /// Refresh token для обновления сессии
    pub refresh_token: String,
    /// Client token (уникален для лаунчера)
    pub client_token: String,
    /// Время истечения access token
    pub expires: DateTime<Utc>,
    /// Дополнительная информация для Ely.by
    #[serde(default)]
    pub ely_data: Option<ElyByData>,
}

impl ElyByCredentials {
    /// Проверка истечения токена
    pub fn is_expired(&self) -> bool {
        // Добавляем буфер 5 минут для безопасности
        self.expires < Utc::now() + Duration::minutes(5)
    }

    /// Проверка валидности токена (с буфером)
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}

/// Хранилище аккаунтов Ely.by
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ElyByAuthStore {
    /// Все аккаунты
    #[serde(default)]
    pub users: std::collections::HashMap<Uuid, ElyByCredentials>,
    /// Аккаунт по умолчанию
    #[serde(default)]
    pub default_user: Option<Uuid>,
    /// Client token (уникален для установки)
    #[serde(default)]
    pub client_token: Option<String>,
}

const ELY_AUTH_JSON: &str = "ely_auth.json";

impl ElyByAuthStore {
    /// Инициализация хранилища
    #[tracing::instrument]
    pub async fn init(
        io_semaphore: &crate::util::fetch::IoSemaphore,
    ) -> crate::Result<Self> {
        use crate::util::fetch::read_json;
        use crate::State;
        
        let state = State::get().await?;
        let auth_path = state.directories.caches_meta_dir().await.join(ELY_AUTH_JSON);
        
        let store = read_json(&auth_path, io_semaphore).await.ok();
        
        Ok(store.unwrap_or_default())
    }

    /// Сохранение хранилища
    #[tracing::instrument(skip(self))]
    pub async fn save(&self) -> crate::Result<()> {
        use crate::util::fetch::write;
        use crate::State;
        
        let state = State::get().await?;
        let auth_path = state.directories.caches_meta_dir().await.join(ELY_AUTH_JSON);
        
        write(&auth_path, &serde_json::to_vec(&self)?, &state.io_semaphore).await?;
        Ok(())
    }

    /// Генерация client token если отсутствует
    pub fn get_or_create_client_token(&mut self) -> String {
        self.client_token
            .get_or_insert_with(|| Uuid::new_v4().to_string())
            .clone()
    }

    /// Получение всех аккаунтов
    pub fn all_accounts(&self) -> Vec<&ElyByCredentials> {
        self.users.values().collect()
    }

    /// Удаление аккаунта
    #[tracing::instrument(skip(self))]
    pub async fn remove(&mut self, id: Uuid) -> crate::Result<Option<ElyByCredentials>> {
        let val = self.users.remove(&id);
        
        // Если удалили default_user, сбрасываем
        if self.default_user == Some(id) {
            self.default_user = self.users.keys().next().copied();
        }
        
        self.save().await?;
        Ok(val)
    }
}
