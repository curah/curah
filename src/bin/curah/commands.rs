use clap::Parser;

mod project;

#[derive(Parser)]
struct CurahCli {
    command: CurahCommand,
}

enum CurahCommand {
    Project,
}

pub fn run() -> Result<(), String> {
    match CurahCli::parse().command {
        CurahCommand::Project => project::run()?,
    }
}
