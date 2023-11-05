use cargo_redox::{args::MainArgs, utils::execute::Execute};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let args = MainArgs::parse();
    let shell = xshell::Shell::new()?;

    args.execute(&shell)?;

    Ok(())
}
