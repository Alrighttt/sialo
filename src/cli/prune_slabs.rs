use crate::util::{BuildSdkError, build_sdk, parse_private_key};
use clap::Parser;
use indexd::Error as IndexdError;
use sia::signing::PrivateKey;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub(crate) enum PruneSlabsError {
    #[error("Failed to create sdk: {0}")]
    NewSdk(#[from] BuildSdkError),
    #[error("Failed to prune slabs: {0}")]
    FetchObject(#[from] IndexdError),
}

#[derive(Parser, Debug)]
#[command(name = "prune_slabs")]
pub(crate) struct PruneSlabsArgs {
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

pub(crate) async fn prune_slabs(args: &PruneSlabsArgs) -> Result<(), PruneSlabsError> {
    let sdk = build_sdk(&args.app_key, &args.indexer_url).await?;
    Ok(sdk.prune_slabs().await?)
}
