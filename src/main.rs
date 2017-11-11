//#[macro_use] extern crate failure_derive;
extern crate clap;
extern crate failure;
extern crate git2;

mod arguments;
mod commit;
mod error;

use std::process::exit;
use std::io::{ Write, stderr };

use arguments::Options;

fn main() {
    if let Err(e) = Options::get_args().run() {
        writeln!(stderr(), "Error encountered while running we-git-you. Error was: \n {}", e)
                .expect("Failed to write out failure of we-git-you to stderr");
        exit(1);
    } else {
        exit(0);
    }
}
