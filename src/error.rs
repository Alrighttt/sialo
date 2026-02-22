use std::path::PathBuf;
use thiserror::Error;

use crate::cli::DeleteError;
use crate::cli::DownloadError;
use crate::cli::ObjectsError;
use crate::cli::PruneSlabsError;
use crate::cli::RegisterError;
use crate::cli::ShareError;
use crate::cli::UploadError;

#[derive(Debug, Error)]
pub(crate) enum Error {
    #[error("Failed to initialize logger: {0}")]
    Log(#[from] LoggerError),
    #[error("Register command failed: {0}")]
    Register(#[from] RegisterError),
    #[error("Upload command failed: {0}")]
    Upload(#[from] UploadError),
    #[error("Download command failed: {0}")]
    Download(#[from] DownloadError),
    #[error("Delete command failed: {0}")]
    Delete(#[from] DeleteError),
    #[error("PruneSlabs command failed: {0}")]
    PruneSlabs(#[from] PruneSlabsError),
    #[error("Share command failed: {0}")]
    Share(#[from] ShareError),
    #[error("Objects command failed: {0}")]
    Objects(#[from] ObjectsError),
}

#[derive(Debug, Error)]
pub(crate) enum LoggerError {
    #[error("failed to create parent directories for log file {path}: {source}")]
    CreateParentDir {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },
    #[error("failed to create log file {path}: {source}")]
    CreateLogFile {
        #[source]
        source: std::io::Error,
        path: PathBuf,
    },
}
