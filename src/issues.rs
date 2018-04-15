pub trait Issue {
    fn name(&self) -> &str;
    fn assignee(&self) -> &str;
    fn description(&self) -> &str;
}

#[derive(Deserialize, Debug)]
struct GitHubLabel {
    name: String,
}

#[derive(Deserialize, Debug)]
struct GitHubUser {
    login: String,
}

#[derive(Deserialize, Debug)]
pub struct GitHubIssue {
    assignee: GitHubUser,
    body: String,
    labels: Vec<GitHubLabel>,
    state: String,
    title: String,
}

#[derive(Deserialize, Debug)]
pub struct GitLabIssue {}

#[derive(Debug)]
pub enum IMIssue {
    GitHub(GitHubIssue),
    GitLab(GitLabIssue),
}

impl Issue for GitHubIssue {
    fn name(&self) -> &str {
        &self.title
    }

    fn assignee(&self) -> &str {
        &self.assignee.login
    }

    fn description(&self) -> &str {
        &self.body
    }
}

impl Issue for IMIssue {
    fn name(&self) -> &str {
        match *self {
            IMIssue::GitHub(ref i) => i.name(),
            _ => unimplemented!(),
        }
    }

    fn assignee(&self) -> &str {
        match *self {
            IMIssue::GitHub(ref i) => i.assignee(),
            _ => unimplemented!(),
        }
    }

    fn description(&self) -> &str {
        match *self {
            IMIssue::GitHub(ref i) => i.description(),
            _ => unimplemented!(),
        }
    }
}
