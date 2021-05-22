use crate::{error, Result};
use regex::Regex;
use url::Url;

// Note that we do not have an SSH-getter currently so this file serves
// only to hold the detect_ssh helper that is used by other detectors.

// detect_ssh determines if the src string matches an SSH-like URL and
// converts it into a net.URL compatible string. This returns nil if the
// string doesn't match the SSH pattern.
//
// This function is tested indirectly via detect_git_test.go
pub(crate) fn detect_ssh(src: &str) -> Result<Option<Url>> {
    lazy_static::lazy_static! {
        // SSH_PATTERN matches SCP-like SSH patterns (user@host:path)
        static ref SSH_PATTERN: Regex = Regex::new(r"^(?:([^@]+)@)?([^:]+):/?(.+)$").unwrap();
    }

    let (user, host, path) = match SSH_PATTERN.captures(src) {
        None => return Ok(None),
        Some(matches) => (
            matches.get(1).map_or("", |m| m.as_str()),
            matches.get(2).map_or("", |m| m.as_str()),
            matches.get(3).map_or("", |m| m.as_str()),
        ),
    };

    let qidx = if let Some(idx) = path.find('?') {
        idx
    } else {
        path.len()
    };

    let mut url = Url::parse(&format!("ssh://{}", host)).map_err(error::detector)?;

    url.set_username(user).map_err(|_| error::detector("error setting username"))?;

    url.set_path(&path[..qidx]);

    if qidx < path.len() {
        url.set_query(Some(&path[(qidx + 1)..]));
    }

    Ok(Some(url))
}
