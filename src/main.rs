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

use crate::branch::build_branch_name;
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: gitx <type> <issue> <summary>");
        return;
    };

    let branch_type = &args[1];
    let issue = &args[2];
    let summary = &args[3];

    let branch_name = build_branch_name(branch_type, issue, summary);
    println!("{}", branch_name);
    let mut list_dir = Command::new("ls");
    // Execute `ls` in the current directory of the program.
    list_dir.status().expect("process failed to execute");

    println!();

    // Change `ls` to execute in the root directory.
    // list_dir.current_dir("/");

    // And then execute `ls` again but in the root directory.
    list_dir.status().expect("process failed to execute");
}
