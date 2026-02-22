use crate::util::{BuildSdkError, build_sdk, from_sia_url, parse_private_key};
use clap::Parser;
use indexd::DownloadOptions;
use indexd::{DownloadError as IndexdDownloadError, Error as IndexdError};
use sia::signing::PrivateKey;
use sia::types::Hash256;
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs::OpenOptions;
use url::Url;

#[derive(Debug, Error)]
pub(crate) enum DownloadError {
    #[error("Failed to create sdk: {0}")]
    NewSdk(#[from] BuildSdkError),
    #[error("Failed to fetch object: {0}")]
    FetchObject(IndexdError),
    #[error("Failed to fetch shared object: {0}")]
    SharedObject(IndexdError),
    #[error("Failed to parse object hash: {0}")]
    ParseHash(String),
    #[error("failed to create parent directories for file {path}: {source}")]
    CreateParentDir {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("failed to create file {path}: {source}")]
    CreateFile {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("Failed to download file: {0}")]
    DownloadFile(IndexdDownloadError),
}

#[derive(Parser, Debug)]
#[command(name = "download")]
pub(crate) struct DownloadArgs {
    /// The URL of the indexer API
    #[arg(
        short = 'u',
        default_value = "https://app.sia.storage",
        global = true,
        env = "INDEXER_URL"
    )]
    pub indexer_url: Url,

    /// The application private key in hex format
    /// ( obtained via ./sialo register )
    #[arg(long, short, env = "APP_KEY", hide_env_values = true, value_parser = parse_private_key)]
    pub app_key: PrivateKey,

    /// A share URL (sia:// or https://) or object hash
    #[arg(value_name = "SOURCE")]
    pub source: String,

    /// The path where the file will be saved
    #[arg(long, short = 'o')]
    pub output_file: PathBuf,
}

pub(crate) async fn download(args: &DownloadArgs) -> Result<(), DownloadError> {
    let sdk = build_sdk(&args.app_key, &args.indexer_url).await?;

    let src = args.source.trim();
    let object = if src.starts_with("sia://") || src.starts_with("https://") {
        let https_url = from_sia_url(src);
        sdk.shared_object(&https_url)
            .await
            .map_err(DownloadError::SharedObject)?
    } else {
        let hash: Hash256 = src
            .parse()
            .map_err(|_| DownloadError::ParseHash(src.to_string()))?;
        sdk.object(&hash)
            .await
            .map_err(DownloadError::FetchObject)?
    };

    // Ensure parent directories exist
    if let Some(parent) = args.output_file.parent() {
        std::fs::create_dir_all(parent).map_err(|e| DownloadError::CreateParentDir {
            source: e,
            path: args.output_file.clone(),
        })?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(args.output_file.clone())
        .await
        .map_err(|e| DownloadError::CreateFile {
            source: e,
            path: args.output_file.clone(),
        })?;

    sdk.download(&mut file, &object, DownloadOptions::default())
        .await
        .map_err(DownloadError::DownloadFile)?;

    Ok(())
}
