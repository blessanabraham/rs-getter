use super::Detector;
use crate::{error, Result};
use async_trait::async_trait;
use url::Url;

/// GitLabDetector implements Detector to detect GitLab URLs and turn
/// them into URLs that the Git Getter can understand.
#[derive(Copy, Clone, Debug)]
pub struct GitLabDetector;

impl GitLabDetector {
    fn detect_http(src: &str) -> Result<(String, bool)> {
        let parts = src.split('/').collect::<Vec<&str>>();
        if parts.len() < 3 {
            return Err(error::detector(
                "GitHub URLs should be gitlab.com/username/repo".to_string(),
            ));
        }

        let url_str = format!("https://{}", parts[..3].join("/"));
        let mut repo_url = Url::parse(&url_str).map_err(error::detector)?;

        if !repo_url.path().ends_with(".git") {
            repo_url.set_path(&format!("{}.git", repo_url.path()));
        }

        if parts.len() > 3 {
            repo_url.set_path(&format!("{}//{}", repo_url.path(), parts[3..].join("/")));
        }

        Ok((format!("git::{}", repo_url.as_str()), true))
    }
}

#[async_trait]
impl Detector for GitLabDetector {
    async fn detect(&self, src: &str, _: &str) -> Result<(String, bool)> {
        if src.is_empty() {
            return Ok(("".to_string(), false));
        }

        if src.starts_with("gitlab.com/") {
            return GitLabDetector::detect_http(src);
        }

        Ok(("".to_string(), false))
    }
}
