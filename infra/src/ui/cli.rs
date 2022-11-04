mod command;

use clap::Parser;
use domain::{EmailAddress, UserFirstName, UserId, UserLastName, UserName};
use interface_adapter::{
    AddUserRequestDTO, Controller, SearchUserRequestDTO, UpdateUserRequestDTO,
};

use crate::{id_generator::IdGenerator, repository_impls::RepositoryImpls};

use self::command::{AddArgs, Cli as ClapCli, Commands, SearchArgs, UpdateArgs};

pub(crate) struct Cli<'r> {
    controller: Controller<'r, RepositoryImpls>,
}

impl<'r> Cli<'r> {
    pub fn new(repositories: &'r RepositoryImpls) -> Self {
        let controller = Controller::new(repositories);
        Self { controller }
    }

    pub(crate) fn run(&self) {
        let clap_cli = ClapCli::parse();

        match &clap_cli.command {
            Some(Commands::Search(args)) => self.process_search_cmd(args),
            Some(Commands::Add(args)) => self.process_add_cmd(args),
            Some(Commands::Update(args)) => self.process_update_cmd(args),
            None => panic!("Invalid command. Run with --help for usage."),
        }
    }

    fn process_search_cmd(&self, args: &SearchArgs) {
        let req = SearchUserRequestDTO {
            email: args.email.clone().map(domain::EmailAddress::new),
            first_name: args.firstname.clone().map(domain::UserFirstName::new),
            last_name: args.lastname.clone().map(domain::UserLastName::new),
        };

        let res = self.controller.search_users(req);
        println!("Found users:\n{:#?}", res.users)
    }

    fn process_add_cmd(&self, args: &AddArgs) {
        let req = AddUserRequestDTO {
            user: domain::User::new(
                UserId::new(IdGenerator::gen()),
                UserName::new(
                    UserFirstName::new(args.firstname.clone()),
                    UserLastName::new(args.lastname.clone()),
                ),
                EmailAddress::new(args.email.clone()),
            ),
        };

        match self.controller.add_user(req) {
            Ok(_res) => println!("Successfully added a user."),
            Err(e) => eprintln!("Failed to add a user: {:?}", e),
        }
    }

    fn process_update_cmd(&self, args: &UpdateArgs) {
        let req = UpdateUserRequestDTO {
            email: domain::EmailAddress::new(args.email.clone()),
            first_name: args.firstname.clone().map(domain::UserFirstName::new),
            last_name: args.lastname.clone().map(domain::UserLastName::new),
        };

        match self.controller.update_user(req) {
            Ok(_res) => println!("Successfully updated a user."),
            Err(e) => println!("Failed to update a user: {:?}", e),
        }
    }
}
