#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct RepoSetting {
    pub repo: String,
    pub branch: String,
}

impl RepoSetting {

    pub fn from_values(repo: String, branch: String) -> RepoSetting {
        RepoSetting {
            repo,
            branch,
        }
    }

}
