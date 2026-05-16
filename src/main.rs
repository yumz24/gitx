// gitx branch feat 123 login-api
// > gitx branch
// input:
//  - type: feat
//  - issue: 123
//  - summary: login-api
//
// output
// feat/#123-login-api
mod branch;
mod cli;
mod error;
mod git;
mod service;

use crate::cli::{Command, parse_args};
use crate::error::GitxError;
use crate::service::execute_branch_create;
use std::env;

fn run() -> Result<(), GitxError> {
    let args = parse_args(env::args())?;

    match args.command {
        Command::Branch => {
            execute_branch_create(&args.branch_type, &args.issue, &args.summary)?;
        }
    };

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
    }
}
