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
mod git;
mod service;

use crate::service::execute_branch_create;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Err(e) = execute_branch_create(args) {
        eprintln!("{}", e)
    }
}
