use crate::cli::GlobalArgs;
use crate::error::LoggerError;
use chrono::{DateTime, TimeDelta, Utc};
use indexd::{Builder, BuilderError, SDK};
use rustls::RootCertStore;
use sia::signing::PrivateKey;
use std::fs::OpenOptions;
use thiserror::Error;
use url::Url;

/// Convert an `https://` share URL to `sia://`.
pub(crate) fn to_sia_url(url: &Url) -> String {
    url.as_str().replacen("https://", "sia://", 1)
}

/// Parse a duration string like "1h", "10d", "4w" or an exact ISO 8601
/// timestamp like "2026-03-30T18:00:00Z" into an absolute DateTime<Utc>.
pub(crate) fn parse_expiry(s: &str) -> Result<DateTime<Utc>, String> {
    let s = s.trim();
    if let Some(num_str) = s.strip_suffix('w') {
        let n: i64 = num_str.parse().map_err(|_| format!("invalid number in '{s}'"))?;
        return Utc::now()
            .checked_add_signed(TimeDelta::weeks(n))
            .ok_or_else(|| "duration overflow".to_string());
    }
    if let Some(num_str) = s.strip_suffix('d') {
        let n: i64 = num_str.parse().map_err(|_| format!("invalid number in '{s}'"))?;
        return Utc::now()
            .checked_add_signed(TimeDelta::days(n))
            .ok_or_else(|| "duration overflow".to_string());
    }
    if let Some(num_str) = s.strip_suffix('h') {
        let n: i64 = num_str.parse().map_err(|_| format!("invalid number in '{s}'"))?;
        return Utc::now()
            .checked_add_signed(TimeDelta::hours(n))
            .ok_or_else(|| "duration overflow".to_string());
    }

    s.parse::<DateTime<Utc>>()
        .map_err(|e| format!("expected duration (1h, 10d, 4w) or ISO 8601 timestamp: {e}"))
}

#[derive(Debug, Error)]
pub(crate) enum ParsePrivateKeyError {
    #[error("Failed to parse app key from hex: {0}")]
    FromHex(#[from] hex::FromHexError),
    #[error("Failed to parse app key from hex, expected 64 bytes, got {0} bytes")]
    HexLength(usize),
}

pub(crate) fn parse_private_key(s: &str) -> Result<PrivateKey, ParsePrivateKeyError> {
    // trim whitespace
    let s = s.trim();

    let bytes = hex::decode(s)?;
    if bytes.len() != 64 {
        return Err(ParsePrivateKeyError::HexLength(bytes.len()));
    };

    let mut arr = [0u8; 64];
    arr.copy_from_slice(&bytes);

    Ok(PrivateKey::from(arr))
}

#[derive(Debug, Error)]
pub(crate) enum BuildSdkError {
    #[error("Failed to create Builder: {0}")]
    NewBuilder(BuilderError),
    #[error("Failed to parse app key: {0}")]
    ParsePrivateKey(#[from] ParsePrivateKeyError),
    #[error("Failed to connect: {0}")]
    Connected(BuilderError),
    #[error("Failed to connect, connected return None")]
    ConnectionFailed,
}

pub(crate) async fn build_sdk(
    app_key: &PrivateKey,
    indexer_url: &Url,
) -> Result<SDK, BuildSdkError> {
    let builder = Builder::new(indexer_url.clone()).map_err(BuildSdkError::NewBuilder)?;

    let sdk = builder
        .connected(app_key, tls_config())
        .await
        .map_err(BuildSdkError::Connected)?
        .ok_or_else(|| BuildSdkError::ConnectionFailed)?;
    Ok(sdk)
}

pub(crate) fn init_logger(cli: &GlobalArgs) -> Result<(), LoggerError> {
    let mut builder = pretty_env_logger::formatted_builder();

    // Set log level from clap args or RUST_LOG if set
    builder.filter_level(cli.log_level);
    builder.parse_env("RUST_LOG");

    // If a log file is specified, redirect output to it
    if let Some(path) = &cli.log_path {
        // Ensure parent directories exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| LoggerError::CreateParentDir {
                source: e,
                path: path.clone(),
            })?;
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| LoggerError::CreateLogFile {
                source: e,
                path: path.clone(),
            })?;

        builder.target(pretty_env_logger::env_logger::Target::Pipe(Box::new(file)));
    };

    builder.init();
    Ok(())
}

/// Build a `rustls::ClientConfig` that trusts the Mozilla WebPKI root
/// certificate set and performs standard server authentication.
pub(crate) fn tls_config() -> rustls::ClientConfig {
    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };

    rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth()
}
