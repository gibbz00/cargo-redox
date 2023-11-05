use crate::{podman::PodmanArgs, repo::RepoArgs, utils::execute::Execute};

#[derive(clap::Parser)]
pub struct MainArgs {
    #[command(subcommand)]
    commands: MainSubCommands,
}

impl Execute for MainArgs {
    fn execute(&self, shell: &xshell::Shell) -> anyhow::Result<()> {
        self.commands.execute(shell)
    }
}

#[derive(clap::Subcommand)]
enum MainSubCommands {
    Podman(PodmanArgs),
    Repo(RepoArgs),
}

impl Execute for MainSubCommands {
    fn execute(&self, shell: &xshell::Shell) -> anyhow::Result<()> {
        match self {
            MainSubCommands::Podman(args) => args.execute(shell),
            MainSubCommands::Repo(args) => args.execute(shell),
        }
    }
}
