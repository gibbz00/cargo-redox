use xshell::Shell;

use crate::utils::execute::Execute;

#[derive(clap::Args)]
pub struct PodmanArgs {}

impl Execute for PodmanArgs {
    fn execute(&self, _shell: &Shell) -> anyhow::Result<()> {
        todo!()
    }
}
