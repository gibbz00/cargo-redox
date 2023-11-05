use xshell::Shell;

use crate::{repo::RepoSubCommands, utils::execute::Execute};

#[derive(clap::Args)]
pub struct PodmanArgs {
    #[command(subcommand)]
    commands: RepoSubCommands,
}

impl Execute for PodmanArgs {
    fn execute(&self, shell: &Shell) -> anyhow::Result<()> {
        self.commands.execute(shell)
    }
}

#[derive(clap::Subcommand)]
pub enum PodmanSubCommands {
    Build,
}

impl Execute for PodmanSubCommands {
    fn execute(&self, _shell: &Shell) -> anyhow::Result<()> {
        todo!()
    }
}
