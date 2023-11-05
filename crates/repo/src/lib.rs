#[derive(clap::Args)]
pub struct RepoArgs {
    #[command(subcommand)]
    commands: RepoSubCommands,
}

#[derive(clap::Subcommand)]
enum RepoSubCommands {
    Clone,
}
