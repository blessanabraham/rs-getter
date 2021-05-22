extern crate std;

use super::Detector;
use crate::{error, Result};
use async_trait::async_trait;
use std::path::{Path, PathBuf};

/// FileDetector implements Detector to detect file paths.
#[derive(Copy, Clone, Debug)]
pub struct FileDetector;

impl FileDetector {
    fn fmt_file_url(path: &str) -> String {
        return if cfg!(target_family = "windows") {
            // Make sure we're using "/" on Windows. URLs are "/"-based.
            let path = path.replace('\\', "/");
            format!("file://{}", path)
        } else {
            // Make sure that we don't start with "/" since we add that below.
            if path.starts_with('/') {
                format!("file://{}", path)
            } else {
                format!("file:///{}", path)
            }
        };
    }
}

#[async_trait]
impl Detector for FileDetector {
    async fn detect(&self, src: &str, pwd: &str) -> Result<(String, bool)> {
        if src.is_empty() {
            return Ok(("".to_string(), false));
        }

        if Path::new(src).is_absolute() {
            return Ok((FileDetector::fmt_file_url(src), true));
        }

        if pwd.is_empty() {
            return Err(error::detector(
                "relative paths require a module with a pwd".to_string(),
            ));
        }

        // Resolve the symlink to it's absolute path
        let mut abs_path = Path::new(pwd).canonicalize().map_err(error::detector)?;

        abs_path.push(src);

        let mut result_path = PathBuf::new();

        let mut path_iter = abs_path.iter().peekable();
        while let Some(component) = path_iter.next() {
            if let Some(next_component) = path_iter.peek() {
                if *next_component == ".." {
                    path_iter.next();
                    continue;
                } else if *next_component == "." {
                    continue;
                }
            }
            result_path.push(&component);
        }

        result_path
            .to_str()
            .ok_or_else(|| error::detector(format!(
                "could not convert {:?} to a string",
                result_path
            )))
            .map(|path| (FileDetector::fmt_file_url(path), true))
    }
}
