use git2::{Repository, Oid};

pub fn get_commits() -> Result<Vec<(String, String, Vec<String>)>, git2::Error> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

    // Collect branch tips
    let mut branch_map = std::collections::HashMap::new();
    for branch in repo.branches(None)? {
        let (branch, _) = branch?;
        //println!("branch: {:?} {:?}", branch.name()?, branch_type);
        if let Some(name) = branch.name()? {
            if let Some(target) = branch.get().target() {
                branch_map.entry(target).or_insert_with(Vec::new).push(name.to_string());
            }
        }
    }

    let mut commits = Vec::new();

    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let hash = oid.to_string()[..7].to_string(); // short hash
        let message = commit.summary().unwrap_or("<no message>").to_string();

        // Check if this commit has any branch pointing to it
        let branches = branch_map.get(&oid).cloned().unwrap_or_default();

        commits.push((hash, message, branches));
    }

    Ok(commits)
}
