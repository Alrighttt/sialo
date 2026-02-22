use crate::util::{BuildSdkError, build_sdk, parse_private_key};
use clap::Parser;
use indexd::Error as IndexdError;
use sia::signing::PrivateKey;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub(crate) enum ObjectsError {
    #[error("Failed to create sdk: {0}")]
    NewSdk(#[from] BuildSdkError),
    #[error("Failed to fetch objects: {0}")]
    FetchObject(#[from] IndexdError),
    #[error("Failed to serialize result to JSON: {0}")]
    SerializeJson(#[from] serde_json::Error),
}

#[derive(Parser, Debug)]
#[command(name = "objects")]
pub(crate) struct ObjectsArgs {
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
    #[arg(long, short, env = "APP_KEY", value_parser = parse_private_key)]
    pub app_key: PrivateKey,
}

pub(crate) async fn objects(args: &ObjectsArgs) -> Result<(), ObjectsError> {
    let sdk = build_sdk(&args.app_key, &args.indexer_url).await?;
    let events = sdk.object_events(None, None).await?;

    for event in events {
        println!("{}:{}", event.id, event.deleted);
    }
    Ok(())
}
