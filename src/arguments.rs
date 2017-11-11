use clap::{ App, ArgMatches, SubCommand };
use error::Results;
use commit;

pub struct Options<'o> {
    matches: ArgMatches<'o>,
}

impl<'o> Options<'o> {
    pub fn get_args() -> Self{
        Self {
            matches: App::new(env!("CARGO_PKG_NAME"))
                         .version(env!("CARGO_PKG_VERSION"))
                         .author(env!("CARGO_PKG_AUTHORS"))
                         .about(env!("CARGO_PKG_DESCRIPTION"))
                         .subcommand(SubCommand::with_name("commit")
                                                .about("Commiting code and it's options"))
                         .subcommand(SubCommand::with_name("github")
                                                .about("Small utilities to work with GitHub"))
                         .get_matches(),
        }
    }

    pub fn run(&self) -> Results<()> {
        if let Some(commit_matches) = self.matches.subcommand_matches("commit") {
            commit::run(commit_matches)
        } else if let Some(_github) = self.matches.subcommand_matches("github") {
            Ok(())
        } else {
            Ok(())
        }
    }
}
