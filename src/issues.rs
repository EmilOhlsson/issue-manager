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
pub struct GitLabLabel {}

#[derive(Deserialize, Debug)]
pub struct GitLabUser {
    name: String,
    username: String,
}

#[derive(Deserialize, Debug)]
pub struct GitLabIssue {
    title: String,
    description: String,
    assignees: Vec<GitLabUser>,
}

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

impl Issue for GitLabIssue {
    fn name(&self) -> &str {
        &self.title
    }

    fn assignee(&self) -> &str {
        &self.assignees
            .get(0)
            .map(|u| u.username.as_ref())
            .unwrap_or("No one")
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl Issue for IMIssue {
    fn name(&self) -> &str {
        match *self {
            IMIssue::GitHub(ref i) => i.name(),
            IMIssue::GitLab(ref i) => i.name(),
        }
    }

    fn assignee(&self) -> &str {
        match *self {
            IMIssue::GitHub(ref i) => i.assignee(),
            IMIssue::GitLab(ref i) => i.assignee(),
        }
    }

    fn description(&self) -> &str {
        match *self {
            IMIssue::GitHub(ref i) => i.description(),
            IMIssue::GitLab(ref i) => i.description(),
        }
    }
}
