pub mod feature;
pub mod user;

#[derive(clap::Parser)]
#[command(name = "minimal-feature-toggle")]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    UserSearch(user::UserSearchArgs),
    UserAdd(user::UserAddArgs),
    UserUpdate(user::UserUpdateArgs),
    FeatureSearch(feature::FeatureSearchArgs),
    FeatureAdd(feature::FeatureAddArgs),
    FeatureGet(feature::FeatureGetArgs),
}
