pub mod execute;
pub mod platform;

#[cfg(test)]
pub mod tests {
    use xshell::Shell;

    pub fn decoupled_shell<F>(f: F) -> anyhow::Result<()>
    where
        F: FnOnce(Shell) -> anyhow::Result<()>,
    {
        let shell = Shell::new()?;

        let temp_dir_guard = shell.create_temp_dir()?;
        shell.change_dir(temp_dir_guard.path());

        f(shell)
    }
}
