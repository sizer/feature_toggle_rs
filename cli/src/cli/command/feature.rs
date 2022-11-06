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
