use super::Detector;
use crate::{error, Result};
use async_trait::async_trait;
use core::str::FromStr;
use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
struct BitBucketResponse {
    scm: String,
}

/// BitBucketDetector implements Detector to detect BitBucket URLs and turn
/// them into URLs that the Git or Hg Getter can understand.
#[derive(Copy, Clone, Debug)]
pub struct BitBucketDetector;

impl BitBucketDetector {
    #[cfg(not(target_arch = "wasm32"))]
    async fn detect_http(src: &str) -> Result<(String, bool)> {
        use hyper::body::Buf;
        use hyper::{Client, Uri};
        use hyper_tls::HttpsConnector;

        let mut url = Url::parse(&format!("https://{}", src)).map_err(error::detector)?;

        // We need to get info on this BitBucket repository to determine whether
        // it is Git or Hg.
        let info_url = format!("https://api.bitbucket.org/2.0/repositories{}", url.path());

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        let uri = Uri::from_str(&info_url).map_err(error::detector)?;

        let resp = client.get(uri).await.map_err(error::detector)?;

        if resp.status() == 403 {
            return Err(error::detector(
                "shorthand BitBucket URL can't be used for private repos, please use a full URL"
                    .to_string(),
            ));
        }

        let body = hyper::body::aggregate(resp)
            .await
            .map_err(error::detector)?;
        let info: BitBucketResponse =
            serde_json::from_reader(body.reader()).map_err(error::detector)?;

        return match info.scm.as_str() {
            "git" => {
                if !url.path().ends_with(".git") {
                    url.set_path(&format!("{}.git", url.path()))
                }

                Ok((format!("git::{}", url), true))
            }
            "hg" => Ok((format!("hg::{}", url), true)),
            _ => Err(error::detector(format!(
                "unknown BitBucket SCM type: {}",
                info.scm
            ))),
        };
    }

    #[cfg(target_arch = "wasm32")]
    async fn detect_http(src: &str) -> Result<(String, bool)> {
        todo!()
    }
}

#[async_trait]
impl Detector for BitBucketDetector {
    async fn detect(&self, src: &str, _: &str) -> Result<(String, bool)> {
        if src.is_empty() {
            return Ok(("".to_string(), false));
        }

        if src.starts_with("bitbucket.org/") {
            return BitBucketDetector::detect_http(src).await;
        }

        Ok(("".to_string(), false))
    }
}
