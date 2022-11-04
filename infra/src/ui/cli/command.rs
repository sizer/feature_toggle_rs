#[derive(clap::Parser)]
#[command(name = "User list")]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Search(SearchArgs),
    Add(AddArgs),
    Update(UpdateArgs),
}

#[derive(clap::Args)]
pub struct SearchArgs {
    #[arg(short, long)]
    pub email: Option<String>,
    #[arg(short, long)]
    pub firstname: Option<String>,
    #[arg(short, long)]
    pub lastname: Option<String>,
}

#[derive(clap::Args)]
pub struct AddArgs {
    #[arg(short, long)]
    pub email: String,
    #[arg(short, long)]
    pub firstname: String,
    #[arg(short, long)]
    pub lastname: String,
}

#[derive(clap::Args)]
pub struct UpdateArgs {
    #[arg(short, long)]
    pub email: String,
    #[arg(short, long)]
    pub firstname: Option<String>,
    #[arg(short, long)]
    pub lastname: Option<String>,
}
