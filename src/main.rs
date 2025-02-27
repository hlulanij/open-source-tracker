struct Contribution {
    username: String,
    repo: String,
    commits: u32,
}

fn main() {
    let contrib = Contribution {
        username: "hlulanij".to_string(),
        repo: "open-source-tracker".to_string(),
        commits: 10,
    };

    println!(
        "{} has made {} commits to {}",
        contrib.username, contrib.commits, contrib.repo
    );
}
