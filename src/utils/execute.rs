use xshell::Shell;

pub trait Execute {
    fn execute(&self, shell: &Shell) -> anyhow::Result<()>;
}
