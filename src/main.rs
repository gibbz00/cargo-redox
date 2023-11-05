use cargo_redox::{podman::PodmanArgs, repo::RepoArgs, utils::execute::Execute};
use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    #[command(subcommand)]
    commands: SubCommands,
}

impl Execute for Args {
    fn execute(&self, shell: &xshell::Shell) -> anyhow::Result<()> {
        self.commands.execute(shell)
    }
}

#[derive(clap::Subcommand)]
enum SubCommands {
    Podman(PodmanArgs),
    Repo(RepoArgs),
}

impl Execute for SubCommands {
    fn execute(&self, shell: &xshell::Shell) -> anyhow::Result<()> {
        match self {
            SubCommands::Podman(args) => args.execute(shell),
            SubCommands::Repo(args) => args.execute(shell),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let shell = xshell::Shell::new()?;

    args.execute(&shell)?;

    Ok(())
}
