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
    FeatureGet(FeatureGetArgs),
}

#[derive(clap::Args)]
pub struct FeatureGetArgs {
    #[arg(short, long)]
    pub user_id: u64,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum FeatureStrategyOption {
    Public,
    Private,
    ABTest,
}

#[derive(clap::Args)]
pub struct FeatureAddArgs {
    #[arg(short, long)]
    pub name: String,
    #[arg(short, long)]
    pub strategy: FeatureStrategyOption,
    // TODO: need validation when strategy is ab-test, percent is required.
    #[arg(short, long)]
    pub percent: Option<u8>,
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
