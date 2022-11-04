#[derive(clap::Parser)]
#[command(name = "minimal-feature-toggle")]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    UserSearch(UserSearchArgs),
    UserAdd(UserAddArgs),
    UserUpdate(UserUpdateArgs),
    FeatureSearch(FeatureSearchArgs),
    FeatureAdd(FeatureAddArgs),
}

#[derive(clap::Args)]
pub struct FeatureAddArgs {
    #[arg(short, long)]
    pub name: String,
}

#[derive(clap::Args)]
pub struct FeatureSearchArgs {
    #[arg(short, long)]
    pub name: Option<String>,
}

#[derive(clap::Args)]
pub struct UserSearchArgs {
    #[arg(short, long)]
    pub email: Option<String>,
    #[arg(short, long)]
    pub firstname: Option<String>,
    #[arg(short, long)]
    pub lastname: Option<String>,
}

#[derive(clap::Args)]
pub struct UserAddArgs {
    #[arg(short, long)]
    pub email: String,
    #[arg(short, long)]
    pub firstname: String,
    #[arg(short, long)]
    pub lastname: String,
}

#[derive(clap::Args)]
pub struct UserUpdateArgs {
    #[arg(short, long)]
    pub email: String,
    #[arg(short, long)]
    pub firstname: Option<String>,
    #[arg(short, long)]
    pub lastname: Option<String>,
}
