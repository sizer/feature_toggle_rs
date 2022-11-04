use domain::{EmailAddress, UserFirstName, UserId, UserLastName, UserName};
use interface_adapter::{
    AddUserRequestDTO, Controller, SearchUserRequestDTO, UpdateUserRequestDTO,
};

use crate::{id_generator::IdGenerator, repository_impls::RepositoryImpls};

pub(crate) struct Cli<'r> {
    controller: Controller<'r, RepositoryImpls>,
}

impl<'r> Cli<'r> {
    pub fn new(repositories: &'r RepositoryImpls) -> Self {
        let controller = Controller::new(repositories);
        Self { controller }
    }

    pub(crate) fn process_cmd(&self) {
        let matches = Self::create_matches();

        if let Some(m) = matches.subcommand_matches("search") {
            self.process_search_cmd(m);
        } else if let Some(m) = matches.subcommand_matches("add") {
            self.process_add_cmd(m);
        } else if let Some(m) = matches.subcommand_matches("update") {
            self.process_update_cmd(m);
        } else {
            panic!("Invalid command. Run with --help for usage.")
        }
    }

    fn process_search_cmd(&self, matches: &clap::ArgMatches) {
        let firstname = matches.value_of("firstname");
        let lastname = matches.value_of("lastname");
        let email = matches.value_of("email");

        let req = SearchUserRequestDTO {
            email: email.map(domain::EmailAddress::new),
            first_name: firstname.map(domain::UserFirstName::new),
            last_name: lastname.map(domain::UserLastName::new),
        };

        let res = self.controller.search_users(req);
        println!("Found users:\n{:#?}", res.users)
    }

    fn process_add_cmd(&self, matches: &clap::ArgMatches) {
        let firstname = matches.value_of("firstname").expect("required");
        let lastname = matches.value_of("lastname").expect("required");
        let email = matches.value_of("email").expect("required");
        let id: u64 = IdGenerator::gen();

        let user = domain::User::new(
            UserId::new(id),
            UserName::new(UserFirstName::new(firstname), UserLastName::new(lastname)),
            EmailAddress::new(email),
        );
        let req = AddUserRequestDTO { user };

        match self.controller.add_user(req) {
            Ok(_res) => println!("Successfully added a user."),
            Err(e) => eprintln!("Failed to add a user: {:?}", e),
        }
    }

    fn process_update_cmd(&self, matches: &clap::ArgMatches) {
        let firstname = matches.value_of("firstname");
        let lastname = matches.value_of("lastname");
        let email = matches.value_of("email").expect("required");

        let req = UpdateUserRequestDTO {
            email: domain::EmailAddress::new(email),
            first_name: firstname.map(domain::UserFirstName::new),
            last_name: lastname.map(domain::UserLastName::new),
        };

        match self.controller.update_user(req) {
            Ok(_res) => println!("Successfully updated a user."),
            Err(e) => println!("Failed to update a user: {:?}", e),
        }
    }

    fn create_matches() -> clap::ArgMatches {
        let firstname_arg = clap::Arg::new("firstname")
            .long("firstname")
            .short('f')
            .takes_value(true);
        let lastname_arg = clap::Arg::new("lastname")
            .long("lastname")
            .short('l')
            .takes_value(true);
        let email_arg = clap::Arg::new("email")
            .long("email")
            .short('e')
            .takes_value(true);

        clap::App::new("User list")
            .version("1.0")
            .subcommand(
                clap::App::new("search")
                    .about("Searches users by name and/or email address")
                    .arg(firstname_arg.clone())
                    .arg(lastname_arg.clone())
                    .arg(email_arg.clone()),
            )
            .subcommand(
                clap::App::new("add")
                    .about("Adds a user")
                    .arg(firstname_arg.clone().required(true))
                    .arg(lastname_arg.clone().required(true))
                    .arg(email_arg.clone().required(true)),
            )
            .subcommand(
                clap::App::new("update")
                    .about("Updates a user's name")
                    .arg(firstname_arg.clone().required(true))
                    .arg(lastname_arg.clone())
                    .arg(email_arg.clone()),
            )
            .get_matches()
    }
}
