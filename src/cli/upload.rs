use crate::util::{BuildSdkError, build_sdk, parse_private_key};
use clap::Parser;
use indexd::Error as IndexdError;
use indexd::UploadError as IndexdUploadError;
use indexd::UploadOptions;
use sia::rhp::SECTOR_SIZE;
use sia::signing::PrivateKey;
use std::io::Write;
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs::File;
use tokio::sync::mpsc;
use url::Url;

#[derive(Debug, Error)]
pub(crate) enum UploadError {
    #[error("Failed to create sdk: {0}")]
    NewSdk(#[from] BuildSdkError),
    #[error("Failed to open file: {0} {1}")]
    OpenFile(PathBuf, std::io::Error),
    #[error("Failed to upload file: {0}")]
    Upload(#[from] IndexdUploadError),
    #[error("Failed to pin object: {0}")]
    Pin(#[from] IndexdError),
}

#[derive(Parser, Debug)]
#[command(name = "upload")]
pub(crate) struct UploadArgs {
    /// The URL of the indexer API
    #[arg(
        short = 'u',
        default_value = "https://app.sia.storage",
        global = true,
        env = "INDEXER_URL"
    )]
    pub indexer_url: Url,

    /// The application private key in hex format
    /// (obtained via ./sialo register)
    #[arg(long, short, env = "APP_KEY", value_parser = parse_private_key)]
    pub app_key: PrivateKey,

    /// The file to upload
    #[arg(value_name = "FILE")]
    pub file: PathBuf,
}

pub(crate) async fn upload(args: &UploadArgs) -> Result<(), UploadError> {
    let sdk = build_sdk(&args.app_key, &args.indexer_url).await?;

    let input = File::open(&args.file)
        .await
        .map_err(|e| UploadError::OpenFile(args.file.clone(), e))?;

    // TODO Read UploadOptions from JSON or arguments
    let mut upload_options = UploadOptions::default();

    // TODO Alright - make this progress bar optional so piping can work
    let (tx, mut rx) = mpsc::unbounded_channel::<()>();
    upload_options.shard_uploaded = Some(tx);

    let file_size = input
        .metadata()
        .await
        .map_err(|e| UploadError::OpenFile(args.file.clone(), e))?
        .len();
    let shards_per_slab = (upload_options.data_shards + upload_options.parity_shards) as u64;
    let slab_data_size = upload_options.data_shards as u64 * SECTOR_SIZE as u64;
    let slab_count = file_size.div_ceil(slab_data_size);
    let total_shards = slab_count * shards_per_slab;

    let progress_task = tokio::spawn(async move {
        let mut done = 0u64;
        while rx.recv().await.is_some() {
            done += 1;
            print!("\rUploaded shards: {done}/{total_shards}");
            let _ = std::io::stdout().flush();
        }
        println!();
    });

    let object = sdk.upload(input, upload_options).await?;
    if let Err(e) = progress_task.await {
        eprintln!("Progress task error: {e}");
    }

    sdk.pin_object(&object).await?;

    println!("Object id: {}", object.id());
    Ok(())
}
