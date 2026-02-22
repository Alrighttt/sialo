use clap::Parser;

pub(crate) mod error;
use error::Error;

pub(crate) mod cli;
use cli::{Command, GlobalArgs};

mod util;
use util::init_logger;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{e}"); // Use Display, not Debug
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Error> {
    rustls::crypto::CryptoProvider::install_default(rustls::crypto::ring::default_provider())
        .expect("install ring CryptoProvider");

    let cli = GlobalArgs::parse();

    init_logger(&cli)?;

    match &cli.command {
        Command::Register(args) => cli::register(&args).await?,
        Command::Upload(args) => cli::upload(&args).await?,
        Command::Download(args) => cli::download(&args).await?,
        Command::Delete(args) => cli::delete(&args).await?,
        Command::PruneSlabs(args) => cli::prune_slabs(&args).await?,
        Command::Share(args) => cli::share(&args).await?,
        Command::Objects(args) => cli::objects(&args).await?,
    }

    Ok(())
}
