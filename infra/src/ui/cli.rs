mod command;

use clap::Parser;
use domain::{EmailAddress, FeatureId, UserFirstName, UserId, UserLastName, UserName};
use interface_adapter::{
    AddFeatureRequestDTO, AddUserRequestDTO, Controller, SearchFeatureRequestDTO,
    SearchUserRequestDTO, UpdateUserRequestDTO,
};

use crate::{id_generator::IdGenerator, repository_impls::RepositoryImpls};

use self::command::{
    Cli as ClapCli, Commands, FeatureAddArgs, FeatureSearchArgs, FeatureStrategyOption,
    UserAddArgs, UserSearchArgs, UserUpdateArgs,
};

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
            Some(Commands::UserSearch(args)) => self.process_search_user_cmd(args),
            Some(Commands::UserAdd(args)) => self.process_add_user_cmd(args),
            Some(Commands::UserUpdate(args)) => self.process_update_user_cmd(args),
            Some(Commands::FeatureSearch(args)) => self.process_search_feature_cmd(args),
            Some(Commands::FeatureAdd(args)) => self.process_add_feature_cmd(args),
            None => panic!("Invalid command. Run with --help for usage."),
        }
    }

    fn process_search_user_cmd(&self, args: &UserSearchArgs) {
        let req = SearchUserRequestDTO {
            email: args.email.clone().map(domain::EmailAddress::new),
            first_name: args.firstname.clone().map(domain::UserFirstName::new),
            last_name: args.lastname.clone().map(domain::UserLastName::new),
        };

        let res = self.controller.search_users(req);
        println!("Found users:\n{:#?}", res.users)
    }

    fn process_add_user_cmd(&self, args: &UserAddArgs) {
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

    fn process_update_user_cmd(&self, args: &UserUpdateArgs) {
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

    fn process_search_feature_cmd(&self, args: &FeatureSearchArgs) {
        let req = SearchFeatureRequestDTO {
            name: args.name.clone().map(domain::FeatureName::new),
        };

        let res = self.controller.search_features(req);
        println!("Found features:\n{:#?}", res.features)
    }

    fn process_add_feature_cmd(&self, args: &FeatureAddArgs) {
        let req = AddFeatureRequestDTO {
            feature: domain::Feature::new(
                FeatureId::new(IdGenerator::gen()),
                domain::FeatureName::new(String::from(args.name.clone())),
                match args.strategy {
                    FeatureStrategyOption::Public => domain::FeatureDistributionStrategy::Public,
                    FeatureStrategyOption::Private => domain::FeatureDistributionStrategy::Private,
                    FeatureStrategyOption::ABTest => domain::FeatureDistributionStrategy::ABTest(
                        args.percent.clone().get_or_insert(0).clone(),
                    ),
                },
            ),
        };

        match self.controller.add_feature(req) {
            Ok(_res) => println!("Successfully added a feature."),
            Err(e) => println!("Failed to add a feature: {:?}", e),
        }
    }
}
