use super::Detector;
use crate::{error, Result};
use async_trait::async_trait;
use url::Url;

/// GitHubDetector implements Detector to detect GitHub URLs and turn
/// them into URLs that the Git Getter can understand.
#[derive(Copy, Clone, Debug)]
pub struct GitHubDetector;

impl GitHubDetector {
    fn detect_http(src: &str) -> Result<(String, bool)> {
        let parts = src.split('/').collect::<Vec<&str>>();
        if parts.len() < 3 {
            return Err(error::detector(
                "GitHub URLs should be github.com/username/repo".to_string(),
            ));
        }

        let url_str = format!("https://{}", parts[..3].join("/"));
        let mut url = Url::parse(&url_str).map_err(error::detector)?;

        if !url.path().ends_with(".git") {
            url.set_path(&format!("{}.git", url.path()));
        }

        if parts.len() > 3 {
            url.set_path(&format!("{}//{}", url.path(), parts[3..].join("/")));
        }

        Ok((format!("git::{}", url.as_str()), true))
    }
}

#[async_trait]
impl Detector for GitHubDetector {
    async fn detect(&self, src: &str, _: &str) -> Result<(String, bool)> {
        if src.is_empty() {
            return Ok(("".to_string(), false));
        }

        if src.starts_with("github.com/") {
            return GitHubDetector::detect_http(src);
        }

        Ok(("".to_string(), false))
    }
}
