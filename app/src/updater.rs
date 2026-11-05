use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubRelease {
    pub tag_name: String,
    pub html_url: String,
    pub assets: Vec<ReleaseAsset>,
}

pub struct AppUpdater {
    pub current_version: String,
    pub repo_owner: String,
    pub repo_name: String,
}

impl AppUpdater {
    pub fn new(current_version: &str, repo_owner: &str, repo_name: &str) -> Self {
        Self {
            current_version: current_version.to_string(),
            repo_owner: repo_owner.to_string(),
            repo_name: repo_name.to_string(),
        }
    }

    pub fn check_latest_release(&self) -> Result<Option<(String, String)>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            self.repo_owner, self.repo_name
        );

        let agent = ureq::AgentBuilder::new()
            .user_agent("HealthifyMe-App-Updater")
            .build();

        let resp = agent.get(&url)
            .set("Accept", "application/vnd.github.v3+json")
            .call()?;

        let release: GithubRelease = resp.into_json()?;

        let latest_tag = release.tag_name.trim_start_matches('v');
        let current_clean = self.current_version.trim_start_matches('v');

        if latest_tag != current_clean {
            let apk_url = release.assets
                .iter()
                .find(|a| a.name.ends_with(".apk"))
                .map(|a| a.browser_download_url.clone())
                .unwrap_or(release.html_url.clone());

            Ok(Some((release.tag_name, apk_url)))
        } else {
            Ok(None)
        }
    }
}
