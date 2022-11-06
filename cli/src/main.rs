mod cli;
///! Infrastructure layer, having main function.
mod controller;

use cli::Cli;
use infra::repository_impls::RepositoryImpls;

fn main() {
    let repo = RepositoryImpls::default();
    let cli = Cli::new(&repo);
    cli.run()
}
