pub fn build_branch_name(branch_type: &str, issue: &str, summary: &str) -> String {
    format!("{}/#{}-{}", branch_type, issue, summary)
}
