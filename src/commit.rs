use clap::ArgMatches;
use error::Results;
use git2::Repository;

pub fn run(args: &ArgMatches) -> Results<()> {
    if let Some(_) = args.value_of("push") {
        Ok(())
    } else {
        commit()
    }
}

fn commit() -> Results<()> {
    Ok(())
}
