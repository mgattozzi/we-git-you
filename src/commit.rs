use clap::ArgMatches;
use error::*;
use git2::{ Commit, ObjectType, Repository };
use std::env;
use std::fs::{ File, remove_file };
use std::io::Read;
use std::process::Command;

pub fn run(args: &ArgMatches) -> Results<()> {
    if let Some(_) = args.value_of("push") {
        Ok(())
    } else {
        commit()
    }
}

fn commit() -> Results<()> {
    let repo = Repository::open(".")?;
    let mut index = repo.index()?;
    let sig = repo.signature()?;
    let tree = repo.find_tree(index.write_tree()?)?;
    let parent = find_last_commit(&repo)?;
    Command::new(env::var("EDITOR")?)
            .arg("COMMIT_MSG")
            .spawn()?
            .wait()?;
    let mut file = File::open("COMMIT_MSG")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    remove_file("COMMIT_MSG")?;
    repo.commit(Some("HEAD"), &sig, &sig, buffer.trim(), &tree, &[&parent])?;
    Ok(())
}

fn find_last_commit(repo: &Repository) -> Results<Commit> {
    repo.head()?
        .resolve()?
        .peel(ObjectType::Commit)?
        .into_commit()
        .map_err(|_| WguError::ParentCommit.into())
}
