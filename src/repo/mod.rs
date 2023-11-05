use xshell::{cmd, Shell};

use crate::utils::execute::Execute;

const REPO_URL: &str = "https://gitlab.redox-os.org/redox-os/redox.git";

#[derive(clap::Args)]
pub struct RepoArgs {
    #[command(subcommand)]
    commands: RepoSubCommands,
}

impl Execute for RepoArgs {
    fn execute(&self, shell: &Shell) -> anyhow::Result<()> {
        self.commands.execute(shell)
    }
}

#[derive(clap::Subcommand)]
enum RepoSubCommands {
    RecursiveClone,
}

impl Execute for RepoSubCommands {
    fn execute(&self, shell: &Shell) -> anyhow::Result<()> {
        cmd!(shell, "git clone {REPO_URL} --origin upstream --recursive").run()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tests::decoupled_shell;

    use super::*;

    #[test]
    #[ignore = "CI only"]
    fn clones_repo() {
        decoupled_shell(|shell| RepoSubCommands::RecursiveClone.execute(&shell)).unwrap()
    }
}
