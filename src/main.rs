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
mod logger;
mod service;

use crate::cli::{Command, parse_branch_args, parse_command, parse_delete_args};
use crate::error::GitxError;
use crate::logger::debug;
use crate::service::execute_branch_create;
use crate::service::execute_branch_delete;
use std::env;

fn run() -> Result<(), GitxError> {
    let mut args = env::args();

    let _program = args.next();

    let command = parse_command(&mut args)?;

    debug(&format!("command: {:?}", command));

    match command {
        Command::Branch => {
            let args = parse_branch_args(&mut args)?;
            let branch_name = execute_branch_create(&args.branch_type, &args.issue, &args.summary)?;
            println!("Created branch: {}", branch_name);
        }
        Command::Delete => {
            let args = parse_delete_args(&mut args)?;
            let branch_name = execute_branch_delete(&args.branch_name)?;
            println!("Deleted branch: {}", branch_name);
        }
    };

    Ok(())
}

fn main() {
    debug("starting gitx");
    if let Err(err) = run() {
        eprintln!("{}", err);
    }
}
