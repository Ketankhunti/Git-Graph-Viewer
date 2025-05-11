use git2::{Repository, Commit};

pub fn get_commits() -> Result<Vec<(String, String)>, git2::Error> {
    let repo = Repository::open(".")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let commits = revwalk
        .filter_map(|oid| {
            let id = oid.ok()?;
            let commit = repo.find_commit(id).ok()?;
            let hash = commit.id().to_string()[..7].to_string();
            let msg = commit.summary().unwrap_or("(no message)").to_string();
            Some((hash, msg))
        })
        .collect();

    Ok(commits)
}
