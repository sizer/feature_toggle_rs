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
