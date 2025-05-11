use git2::Repository;
use git2::Commit;
use git2::Sort;
// struct MyRepo(git2::Repository);

// impl std::fmt::Debug for MyRepo {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "Git repository at: {:?}", Commit::id(&self))
//     }
// }


fn get_repo() -> Repository {
    match Repository::open("D:\\Git Graph Viewer\\Git_Graph_Viewer") {
        Ok(repo) => repo,
        Err(e) => {
            println!("Failed to open repository: {}", e);
            Repository::init("D:\\Git Graph Viewer\\Git_Graph_Viewer").expect("Could not initialize the repository")
        }
    }
}


fn main() {
    let repo = get_repo();
    // let my_repo = MyRepo(repo);
    let mut revwalk = repo.revwalk().unwrap();

    let _ = revwalk.push_head();

    let _ = revwalk.set_sorting(Sort::TOPOLOGICAL);

    for oid_result in revwalk {
        let oid = oid_result.unwrap();
        let commit = repo.find_commit(oid).unwrap();
        
        println!(
            "{} | {} | {}",
            &commit.id().to_string()[..7],                // Short commit hash
            commit.author().name().unwrap_or("Unknown"),  // Author name
            commit.summary().unwrap_or("(no message)")    // Commit message
        );
    }



}
