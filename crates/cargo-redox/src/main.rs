use clap::Parser;

use cargo_redox_podman::PodmanArgs;
use cargo_redox_repo::RepoArgs;

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    commands: SubCommands,
}

#[derive(clap::Subcommand)]
enum SubCommands {
    Podman(PodmanArgs),
    Repo(RepoArgs),
}

fn main() {
    let _args = Args::parse();
}
