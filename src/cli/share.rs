use crate::util::{BuildSdkError, build_sdk, parse_private_key};
use chrono::{DateTime, TimeDelta, Utc};
use clap::Parser;
use indexd::Error as IndexdError;
use sia::signing::PrivateKey;
use sia::types::Hash256;
use thiserror::Error;
use url::Url;

/// Parse a duration string like "1h", "10d", "4w" or an exact ISO 8601
/// timestamp like "2026-03-30T18:00:00Z" into an absolute DateTime<Utc>.
fn parse_expiry(s: &str) -> Result<DateTime<Utc>, String> {
    // Try relative duration first: a number followed by h/d/w
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

    // Fall back to exact ISO 8601 timestamp
    s.parse::<DateTime<Utc>>()
        .map_err(|e| format!("expected duration (1h, 10d, 4w) or ISO 8601 timestamp: {e}"))
}

#[derive(Debug, Error)]
pub(crate) enum ShareError {
    #[error("Failed to create sdk: {0}")]
    NewSdk(#[from] BuildSdkError),
    #[error("Failed to fetch object: {0}")]
    FetchObject(IndexdError),
    #[error("Failed to share object: {0}")]
    ShareObject(IndexdError),
}

#[derive(Parser, Debug)]
#[command(name = "share", after_long_help = "\
Examples:
  Share for 4 weeks:
    sialo share -s <OBJECT_HASH> -t 4w

  Share for 10 days:
    sialo share -s <OBJECT_HASH> -t 10d

  Share for 1 hour:
    sialo share -s <OBJECT_HASH> -t 1h

  Share until a specific date:
    sialo share -s <OBJECT_HASH> -t 2026-12-31T23:59:59Z")]
pub(crate) struct ShareArgs {
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

    /// The object hash of the file to share
    /// ( obtained via ./sialo upload )
    #[arg(long, short = 's')]
    pub object_hash: Hash256,

    /// The expiration time for the share link
    /// (e.g. 1h, 10d, 4w, or 2026-03-30T18:00:00Z)
    #[arg(long, short = 't', value_parser = parse_expiry)]
    pub share_until: DateTime<Utc>,
}

pub(crate) async fn share(args: &ShareArgs) -> Result<(), ShareError> {
    let sdk = build_sdk(&args.app_key, &args.indexer_url).await?;

    let object = sdk
        .object(&args.object_hash)
        .await
        .map_err(ShareError::FetchObject)?;
    let url = sdk
        .share_object(&object, args.share_until)
        .map_err(ShareError::ShareObject)?;
    println!("{}", url);
    Ok(())
}
