use clap::{Parser, Subcommand};
use log::LevelFilter;
use std::path::PathBuf;

mod delete;
mod download;
mod objects;
mod prune_slabs;
mod register;
mod share;
mod upload;

pub(crate) use delete::{DeleteArgs, DeleteError, delete};
pub(crate) use download::{DownloadArgs, DownloadError, download};
pub(crate) use objects::{ObjectsArgs, ObjectsError, objects};
pub(crate) use prune_slabs::{PruneSlabsArgs, PruneSlabsError, prune_slabs};
pub(crate) use register::{RegisterArgs, RegisterError, register};
pub(crate) use share::{ShareArgs, ShareError, share};
pub(crate) use upload::{UploadArgs, UploadError, upload};

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Register a new application key
    ///
    /// Outputs:
    /// - Application key (hex)
    Register(RegisterArgs),

    /// Upload a file
    ///
    /// Outputs:
    /// - Object hash (hex)
    Upload(UploadArgs),

    /// Download a file
    Download(DownloadArgs),

    /// Delete an object from the indexer
    Delete(DeleteArgs),

    /// Generates a shared URL for an object.
    ///
    /// Outputs:
    /// - Share URL
    Share(ShareArgs),

    /// Prune unused slabs from the indexer.
    PruneSlabs(PruneSlabsArgs),

    /// Retrieve a list of objects from the indexer.
    Objects(ObjectsArgs),
}

/// A CLI for uploading, downloading, and managing files on Sia
#[derive(Parser, Debug)]
#[command(name = "sialo")]
#[command(version, about, long_about = None)]
pub(crate) struct GlobalArgs {
    #[command(subcommand)]
    pub command: Command,

    /// Log level (error, warn, info, debug, trace)
    #[arg(long = "log-level", short = 'l', default_value = "info", global = true)]
    pub log_level: LevelFilter,

    /// Path to write logs to (stdout if not set)
    #[arg(long = "log-path", env = "LOG_PATH", global = true)]
    pub log_path: Option<PathBuf>,
}
