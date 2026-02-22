use crate::util::tls_config;
use clap::Parser;
use indexd::app_client::RegisterAppRequest;
use indexd::{Builder, BuilderError};
use sia::types::Hash256;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub(crate) enum RegisterError {
    #[error("Failed to create Builder: {0}")]
    NewBuilder(BuilderError),
    #[error("Failed to parse AppMetadata: {0}")]
    ParseAppMetadata(#[from] ParseAppRequestError),
    #[error("request_connection failed: {0}")]
    RequestConnection(BuilderError),
    #[error("wait_for_approval failed: {0}")]
    WaitForApproval(BuilderError),
    #[error("Failed to register: {0}")]
    Register(BuilderError),
}

#[derive(Parser, Debug)]
#[command(name = "register", about)]
pub(crate) struct RegisterArgs {
    /// The URL of the indexer API
    #[arg(
        long = "indexer-url",
        short = 'u',
        default_value = "https://app.sia.storage",
        global = true,
        env = "INDEXER_URL"
    )]
    pub indexer_url: Url,

    /// The mnemonic used to derive the application key
    ///
    /// Can be provided via:
    ///   --seed-phrase
    ///   or the SEED_PHRASE environment variable
    ///
    /// (Use `sialo generate-seed` to generate a new one)
    #[arg(long = "seed-phrase", short = 's', env = "SEED_PHRASE")]
    pub seed_phrase: String,

    /// The path to a AppMetadata JSON file.
    ///
    /// (if this is not set, sialo's default will be used)
    #[arg(long, short, env = "APP_METADATA")]
    pub app_metadata: Option<PathBuf>,
}

fn default_app_request() -> RegisterAppRequest {
    RegisterAppRequest {
        app_id: Hash256::from_str(
            "c0000000000000000000000000000000000000000000000000000000000000de",
        )
        .expect("A valid Hash256"),
        name: "sialo".to_string(),
        description: "A CLI for uploading, downloading, and managing files on Sia".to_string(),
        service_url: "https://example.com".parse().expect("A valid URL"),
        logo_url: None,
        callback_url: None,
    }
}

#[derive(Debug, Error)]
pub(crate) enum ParseAppRequestError {
    #[error("Failed to read JSON: {0}")]
    Read(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),
}

/// Reads a RegisterAppRequest from the file path provided in RegisterArgs.request_json
/// or returns sialo's default configuration
fn parse_app_request(args: &RegisterArgs) -> Result<RegisterAppRequest, ParseAppRequestError> {
    if let Some(path) = &args.app_metadata {
        let data = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    } else {
        Ok(default_app_request())
    }
}

pub(crate) async fn register(args: &RegisterArgs) -> Result<(), RegisterError> {
    let builder =
        Builder::new(args.indexer_url.clone()).map_err(|e| RegisterError::NewBuilder(e))?;

    let request = parse_app_request(&args)?;
    let builder = builder
        .request_connection(&request)
        .await
        .map_err(|e| RegisterError::RequestConnection(e))?;

    println!(
        "Please approve the app connection by visiting the following URL: {}",
        builder.response_url()
    );

    let builder = builder
        .wait_for_approval()
        .await
        .map_err(|e| RegisterError::WaitForApproval(e))?;

    let sdk = builder
        .register(&args.seed_phrase, tls_config())
        .await
        .map_err(|e| RegisterError::Register(e))?;

    println!("Application key: {}", hex::encode(sdk.app_key()));
    Ok(())
}
