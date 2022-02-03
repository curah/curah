use clap::{Parser, Subcommand};

mod project;

#[derive(Parser)]
struct CurahCli {
    #[clap(subcommand)]
    command: CurahCommand,
}

#[derive(Subcommand)]
enum CurahCommand {
    Project,
}

pub fn run() -> Result<(), String> {
    match CurahCli::parse().command {
        CurahCommand::Project => project::run(),
    }
}
