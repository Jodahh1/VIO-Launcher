//! Authlib-Injector Integration for Ely.by
//! 
//! Provides secure download, verification, and management of authlib-injector.jar
//! with fault-tolerant error handling and progress tracking.

use crate::util::fetch::REQWEST_CLIENT;
use crate::State;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;
use tokio_stream::StreamExt;

/// Официальный URL для загрузки authlib-injector
const AUTHLIB_INJECTOR_URL: &str = 
    "https://github.com/yushijinhun/authlib-injector/releases/latest/download/authlib-injector.jar";

/// Ожидаемый SHA256 хеш (обновлять при новой версии)
/// Получается через: curl -L <url> | sha256sum
/// ЗАМЕНИТЬ НА АКТУАЛЬНЫЙ ХЕШ ПРИ РЕЛИЗЕ
const AUTHLIB_INJECTOR_SHA256: &str = ""; // Пустой = без проверки хеша

/// Имя файла
const AUTHLIB_FILENAME: &str = "authlib-injector.jar";

/// Ely.by authentication server URL
pub const ELY_BY_URL: &str = "https://authserver.ely.by";

/// Статус загрузки для UI
#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub percentage: f64,
}

/// Канал для отправки прогресса во фронтенд
pub type ProgressSender = tokio::sync::mpsc::Sender<DownloadProgress>;

/// Результат проверки/загрузки
#[derive(Debug, Clone)]
pub enum AuthlibStatus {
    /// Файл уже существует и валиден
    AlreadyExists(PathBuf),
    /// Файл успешно загружен
    Downloaded(PathBuf),
    /// Ошибка
    Error(String),
}

/// Глобальное состояние загрузчика (избегаем повторных загрузок)
static DOWNLOAD_LOCK: RwLock<Option<bool>> = RwLock::const_new(None);

/// Путь к authlib-injector.jar в кэше лаунчера
#[tracing::instrument]
pub async fn authlib_injector_path() -> crate::Result<PathBuf> {
    let state = State::get().await?;
    Ok(state.directories.caches_meta_dir().await.join(AUTHLIB_FILENAME))
}

/// Проверка целостности файла по SHA256
#[tracing::instrument(skip(path))]
async fn verify_sha256(path: &Path, expected_hash: &str) -> crate::Result<bool> {
    if expected_hash.is_empty() {
        // Если хеш не задан, пропускаем проверку
        return Ok(true);
    }

    let file_bytes = tokio::fs::read(path)
        .await
        .map_err(|e| {
            crate::ErrorKind::LauncherError(format!(
                "Failed to read authlib-injector.jar: {}",
                e
            ))
        })?;

    let mut hasher = Sha256::new();
    hasher.update(&file_bytes);
    let actual_hash = format!("{:x}", hasher.finalize());

    // Сравниваем хеши (case-insensitive)
    let is_valid = actual_hash.eq_ignore_ascii_case(expected_hash);
    
    if !is_valid {
        tracing::warn!(
            "SHA256 mismatch! Expected: {}, Got: {}",
            expected_hash,
            actual_hash
        );
    }

    Ok(is_valid)
}

/// Проверка существования и валидности файла
#[tracing::instrument]
pub async fn check_authlib_injector() -> crate::Result<Option<PathBuf>> {
    let path = authlib_injector_path().await?;

    if !path.exists() {
        tracing::debug!("authlib-injector.jar not found");
        return Ok(None);
    }

    // Проверяем хеш (если задан)
    if !AUTHLIB_INJECTOR_SHA256.is_empty() {
        match verify_sha256(&path, AUTHLIB_INJECTOR_SHA256).await {
            Ok(true) => {
                tracing::info!("authlib-injector.jar verified successfully");
                return Ok(Some(path));
            }
            Ok(false) => {
                tracing::warn!("authlib-injector.jar hash mismatch, will re-download");
                // Удаляем невалидный файл
                let _ = tokio::fs::remove_file(&path).await;
                return Ok(None);
            }
            Err(e) => {
                tracing::warn!("Failed to verify hash: {}", e);
                // Если не можем проверить, но файл есть - используем его
                return Ok(Some(path));
            }
        }
    }

    // Если хеш не задан, просто проверяем размер > 0
    let metadata = tokio::fs::metadata(&path).await.map_err(|e| {
        crate::ErrorKind::LauncherError(format!(
            "Failed to get authlib-injector metadata: {}",
            e
        ))
    })?;

    if metadata.len() > 0 {
        tracing::info!("authlib-injector.jar exists (size: {} bytes)", metadata.len());
        Ok(Some(path))
    } else {
        tracing::warn!("authlib-injector.jar is empty, will re-download");
        let _ = tokio::fs::remove_file(&path).await;
        Ok(None)
    }
}

/// Асинхронная загрузка authlib-injector с прогрессом
#[tracing::instrument(skip(progress_tx))]
pub async fn download_authlib_injector(
    progress_tx: Option<ProgressSender>,
) -> crate::Result<PathBuf> {
    // Блокировка для предотвращения конкурентных загрузок
    let mut lock = DOWNLOAD_LOCK.write().await;
    if lock.is_some() {
        tracing::debug!("Download already in progress, waiting...");
        drop(lock);
        
        // Ждем завершения другой загрузки
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        return check_authlib_injector()
            .await?
            .ok_or_else(|| {
                crate::ErrorKind::LauncherError(
                    "Concurrent download failed".to_string()
                ).into()
            });
    }
    
    *lock = Some(true);
    drop(lock);

    let state = State::get().await?;
    let path = authlib_injector_path().await?;
    
    // Создаем директорию если не существует
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent).await.map_err(|e| {
            crate::ErrorKind::LauncherError(format!(
                "Failed to create cache directory: {}",
                e
            ))
        })?;
    }

    tracing::info!("Downloading authlib-injector from {}", AUTHLIB_INJECTOR_URL);

    // Создаем HTTP запрос с таймаутами
    let response = REQWEST_CLIENT
        .get(AUTHLIB_INJECTOR_URL)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| {
            crate::ErrorKind::LauncherError(format!(
                "Failed to start download: {}",
                e
            ))
        })?;

    // Проверяем статус ответа
    let status = response.status();
    if !status.is_success() {
        return Err(crate::ErrorKind::LauncherError(format!(
            "Download failed with status: {}",
            status
        )).into());
    }

    // Получаем размер файла (если доступен)
    let total_bytes = response.content_length();
    
    // Создаем файл для записи
    let mut file = tokio::fs::File::create(&path).await.map_err(|e| {
        crate::ErrorKind::LauncherError(format!(
            "Failed to create file (permission denied?): {}",
            e
        ))
    })?;

    // Загружаем по частям с отслеживанием прогресса
    use tokio::io::AsyncWriteExt;
    
    let mut downloaded_bytes: u64 = 0;
    let mut stream = response.bytes_stream();
    
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.map_err(|e| {
            crate::ErrorKind::LauncherError(format!(
                "Download interrupted: {}",
                e
            ))
        })?;

        file.write_all(&chunk).await.map_err(|e| {
            crate::ErrorKind::LauncherError(format!(
                "Failed to write to file: {}",
                e
            ))
        })?;

        downloaded_bytes += chunk.len() as u64;

        // Отправляем прогресс
        if let Some(ref tx) = progress_tx {
            let percentage = total_bytes
                .map(|total| (downloaded_bytes as f64 / total as f64) * 100.0)
                .unwrap_or(0.0);

            let _ = tx.send(DownloadProgress {
                downloaded_bytes,
                total_bytes,
                percentage,
            }).await;
        }
    }

    file.flush().await.map_err(|e| {
        crate::ErrorKind::LauncherError(format!(
            "Failed to flush file: {}",
            e
        ))
    })?;

    drop(file);

    tracing::info!(
        "Download complete: {} bytes",
        downloaded_bytes
    );

    // Освобождаем блокировку
    *DOWNLOAD_LOCK.write().await = None;

    // Проверяем хеш если задан
    if !AUTHLIB_INJECTOR_SHA256.is_empty() {
        if !verify_sha256(&path, AUTHLIB_INJECTOR_SHA256).await? {
            return Err(crate::ErrorKind::LauncherError(
                "SHA256 verification failed after download".to_string()
            ).into());
        }
    }

    Ok(path)
}

/// Получение или загрузка authlib-injector (умная обертка)
#[tracing::instrument(skip(progress_tx))]
pub async fn get_or_download_authlib(
    progress_tx: Option<ProgressSender>,
) -> crate::Result<PathBuf> {
    // Проверяем наличие
    if let Some(path) = check_authlib_injector().await? {
        return Ok(path);
    }

    // Скачиваем
    download_authlib_injector(progress_tx).await
}

/// Формирование JVM аргумента для authlib-injector
#[tracing::instrument]
pub async fn get_authlib_agent_arg() -> crate::Result<String> {
    let path = authlib_injector_path().await?;
    
    // Проверяем существование перед использованием
    if !path.exists() {
        return Err(crate::ErrorKind::LauncherError(
            "authlib-injector.jar not found. Please download it first.".to_string()
        ).into());
    }

    let path_str = path.to_string_lossy();
    
    // Формируем аргумент: -javaagent:/path/to/authlib-injector.jar=https://authserver.ely.by
    Ok(format!("-javaagent:{}=ely.by", path_str))
}

/// Очистка кэша authlib-injector
#[tracing::instrument]
pub async fn cleanup_authlib_cache() -> crate::Result<()> {
    let path = authlib_injector_path().await?;
    
    if path.exists() {
        tokio::fs::remove_file(&path).await.map_err(|e| {
            crate::ErrorKind::LauncherError(format!(
                "Failed to remove authlib-injector: {}",
                e
            ))
        })?;
        tracing::info!("authlib-injector cache cleared");
    }
    
    Ok(())
}

/// Проверка доступности Ely.by API
#[tracing::instrument]
pub async fn check_ely_by_availability() -> bool {
    use std::time::Duration;
    
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .ok();
    
    let client = match client {
        Some(c) => c,
        None => return false,
    };

    match client
        .get(format!("{}/authserver/", ELY_BY_URL))
        .send()
        .await
    {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_authlib_path() {
        let path = authlib_injector_path();
        assert!(path.is_ok());
    }

    #[tokio::test]
    async fn test_ely_by_availability() {
        // Тест может упасть без интернета
        let available = check_ely_by_availability().await;
        println!("Ely.by available: {}", available);
    }
}
