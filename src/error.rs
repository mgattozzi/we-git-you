use failure::{ Fail, Error };

use std::fmt;

pub type Results<T> = Result<T, Error>;

#[derive(Debug)]
pub enum WguError {
    ParentCommit
}

impl Fail for WguError {
    fn fail(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::WguError::*;
        match *self {
            ParentCommit => fmt.write_str("Failed to get parent commit")
        }
    }
}
