use cargo_redox_podman::PodmanArgs;
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    sub_command: SubCommands,
}

#[derive(clap::Subcommand)]
enum SubCommands {
    Podman(PodmanArgs),
}

fn main() {
    let _args = Args::parse();
}
