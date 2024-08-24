use std::fmt::{Display, Formatter, Result};

struct GitHub {
    owner: String,
    repo: String,
}

impl Display for GitHub {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}/{}", self.owner, self.repo)
    }
}

fn main() {
    let github = GitHub {
        owner: "rust-lang".to_string(),
        repo: "rust".to_string(),
    };

    println!("{}", github);
}
