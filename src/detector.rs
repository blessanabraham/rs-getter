use crate::{error, Result};
use async_trait::async_trait;
use regex::Regex;
use std::fmt;
use std::sync::Arc;
use url::Url;

cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        mod file;
        pub use file::FileDetector;
    }
}

mod bitbucket;
pub use bitbucket::BitBucketDetector;

mod git;
pub use git::GitDetector;

mod ssh;

mod github;
pub use github::GitHubDetector;

mod gitlab;
pub use gitlab::GitLabDetector;

lazy_static::lazy_static! {
    #[allow(missing_docs)]
    pub static ref DETECTORS: Arc<Vec<Box<dyn Detector>>> = Arc::new(vec![
        Box::new(GitHubDetector),
        Box::new(GitLabDetector),
        Box::new(GitDetector),
        Box::new(BitBucketDetector),
        #[cfg(not(target_arch = "wasm32"))]
        Box::new(FileDetector),
    ]);
}

/// Detector defines the interface that an invalid URL or a URL with a blank
/// scheme is passed through in order to determine if its shorthand for
/// something else well-known.
#[async_trait]
pub trait Detector: fmt::Debug + Sync + Send + 'static {
    /// Detect will detect whether the string matches a known pattern to
    /// turn it into a proper URL.
    async fn detect(&self, src: &str, pwd: &str) -> Result<(String, bool)>;
}

fn get_forced_getter(src: &str) -> (&str, &str) {
    lazy_static::lazy_static! {
        static ref FORCED_REGEXP: Regex = Regex::new(r"^([A-Za-z0-9]+)::(.+)$").unwrap();
    }

    let (forced, get_src) = match FORCED_REGEXP.captures(src) {
        Some(matches) => (
            matches.get(1).map_or("", |m| m.as_str()),
            matches.get(2).map_or(src, |m| m.as_str()),
        ),
        None => ("", src),
    };

    (forced, get_src)
}

// source_dir_subdir takes a source URL and returns a tuple of the URL without
// the subdir and the subdir.
//
// ex:
//   dom.com/path/?q=p               => dom.com/path/?q=p, ""
//   proto://dom.com/path//*?q=p     => proto://dom.com/path?q=p, "*"
//   proto://dom.com/path//path2?q=p => proto://dom.com/path?q=p, "path2"
//
fn source_dir_subdir(src: &str) -> (String, String) {
    // URL might contain another url in query parameters
    let mut stop = src.len();
    if let Some(idx) = src.find('?') {
        stop = idx
    }

    // Calculate an offset to avoid accidentally marking the scheme
    // as the dir.
    let offset = if let Some(idx) = src[..stop].find("://") {
        idx + 3
    } else {
        0
    };

    // First see if we even have an explicit subdir
    let idx = match src[offset..stop].find("//") {
        Some(idx) => idx + offset,
        None => return (src.to_string(), "".to_string()),
    };

    let subdir = src[idx + 2..].to_owned();
    let mut src = src[..idx].to_owned();

    // Next, check if we have query parameters and push them onto the
    // URL.
    if let Some(idx) = subdir.find('?') {
        let query = &subdir[idx..];
        let subdir = subdir[..idx].to_owned();
        src = format!("{}{}", src, query);
        return (src, subdir);
    }

    (src, subdir)
}

/// Detect turns a source string into another source string if it is
/// detected to be of a known pattern.
///
/// The third parameter should be the list of detectors to use in the
/// order to try them. If you don't want to configure this, just use
/// the global Detectors variable.
///
/// This is safe to be called with an already valid source string: Detect
/// will just return it.
pub async fn detect(src: &str, pwd: &str, detectors: &[Box<dyn Detector>]) -> Result<String> {
    let (get_force, get_src) = get_forced_getter(src);

    // Separate out the subdir if there is one, we don't pass that to detect
    let (get_src, mut subdir) = source_dir_subdir(get_src);

    if let Ok(url) = Url::parse(&get_src) {
        if url.scheme() != "" {
            return Ok(src.to_owned());
        }
    }

    for detector in detectors {
        let (result, ok) = detector.detect(&get_src, pwd).await?;
        if !ok {
            continue;
        }

        let (detect_force, result) = get_forced_getter(&result);
        let (mut result, detect_subdir) = source_dir_subdir(result);

        // If we have a subdir from the detection, then prepend it to our
        // requested subdir.
        if !detect_subdir.is_empty() {
            if !subdir.is_empty() {
                subdir = format!("{}/{}", &detect_subdir, subdir)
            } else {
                subdir = detect_subdir;
            }
        }

        if !subdir.is_empty() {
            result = Url::parse(&result)
                .map(|mut url| {
                    url.set_path(&format!("{}//{}", url.path(), subdir));
                    url.to_string()
                })
                .map_err(error::detector)?;
        }

        // Preserve the forced getter if it exists. We try to use the
        // original set force first, followed by any force set by the
        // detector.
        if !get_force.is_empty() {
            result = format!("{}::{}", get_force, result);
        } else if !detect_force.is_empty() {
            result = format!("{}::{}", detect_force, result);
        }

        return Ok(result);
    }

    Err(error::detector(format!("Invalid source string `{}`", src)))
}
