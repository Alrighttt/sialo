use crate::util::{BuildSdkError, build_sdk, parse_private_key};
use clap::Parser;
use indexd::Error as IndexdError;
use sia::signing::PrivateKey;
use sia::types::Hash256;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub(crate) enum DeleteError {
    #[error("Failed to create sdk: {0}")]
    NewSdk(#[from] BuildSdkError),
    #[error("Failed to delete object: {0}")]
    FetchObject(#[from] IndexdError),
}

#[derive(Parser, Debug)]
#[command(name = "delete")]
pub(crate) struct DeleteArgs {
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

    /// The object hash of the file to delete
    /// ( obtained via ./sialo upload )
    #[arg(value_name = "OBJECT_HASH")]
    pub object_hash: Hash256,
}

pub(crate) async fn delete(args: &DeleteArgs) -> Result<(), DeleteError> {
    let sdk = build_sdk(&args.app_key, &args.indexer_url).await?;
    Ok(sdk.delete_object(&args.object_hash).await?)
}
