use crate::{branch::build_branch_name, git::git_branch};

pub fn execute_branch_create(args: Vec<String>) -> Result<String, String> {
    if args.len() != 4 {
        return Err("Usage: gitx <type> <issue> <summary>".to_string());
    };

    let branch_type = &args[1];
    let issue = &args[2];
    let summary = &args[3];
    let branch_name: String = build_branch_name(branch_type, issue, summary);

    git_branch(branch_name)
}
