pub trait Issue {
    fn name(&self) -> &str;
    fn assignee(&self) -> &str;
    fn description(&self) -> &str;
}

#[derive(Deserialize, Debug)]
struct GithubLabel {
    name: String,
}

#[derive(Deserialize, Debug)]
struct GithubUser {
    login: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubIssue {
    assignee: GithubUser,
    body: String,
    labels: Vec<GithubLabel>,
    state: String,
    title: String,
}

impl Issue for GithubIssue {
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
