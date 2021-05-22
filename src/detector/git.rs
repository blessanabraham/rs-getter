use super::Detector;
use crate::detector::ssh::detect_ssh;
use crate::Result;
use async_trait::async_trait;

/// GitDetector implements Detector to detect Git SSH URLs such as
/// git@host.com:dir1/dir2 and converts them to proper URLs.
#[derive(Copy, Clone, Debug)]
pub struct GitDetector;

#[async_trait]
impl Detector for GitDetector {
    async fn detect(&self, src: &str, _: &str) -> Result<(String, bool)> {
        if src.is_empty() {
            return Ok(("".to_string(), false));
        }

        let url = match detect_ssh(src)? {
            Some(u) => u,
            None => return Ok(("".to_string(), false)),
        };

        // We require the username to be "git" to assume that this is a Git URL
        if url.username() != "git" {
            return Ok(("".to_string(), false));
        }

        Ok((format!("git::{}", url.as_str()), true))
    }
}
