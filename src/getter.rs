use crate::{ClientMode, Result};
use async_trait::async_trait;
use futures_util::Stream;
use std::collections::BTreeMap;
use std::fmt;
use std::sync::Arc;

mod http;
pub(crate) use http::HttpGetter;

mod git;
pub(crate) use git::GitGetter;

lazy_static::lazy_static! {
    #[allow(missing_docs)]
    pub static ref GETTERS: Arc<BTreeMap<&'static str, Box<dyn Getter>>> = {
        let mut map: BTreeMap<&str, Box<dyn Getter>> = BTreeMap::new();

        map.insert("git", Box::new(GitGetter));
        map.insert("http", Box::new(HttpGetter));
        map.insert("https", Box::new(HttpGetter));

        Arc::new(map)
    };
}

// Getter defines the interface that schemes must implement to download
// things.
#[async_trait]
pub trait Getter: fmt::Debug + Sync + Send + 'static {
    /// Get downloads the given URL into the given directory. This always
    /// assumes that we're updating and gets the latest version that it can.
    ///
    /// The directory may already exist (if we're updating). If it is in a
    /// format that isn't understood, an error should be returned. Get shouldn't
    /// simply nuke the directory.
    async fn get(&self, url: &str) -> Result<&dyn Stream<Item = &[u8]>>;

    /// get_file downloads the give URL into the given path. The URL must
    /// reference a single file. If possible, the Getter should check if
    /// the remote end contains the same file and no-op this operation.
    async fn get_file(&self, url: &str) -> Result<&dyn Stream<Item = &[u8]>>;

    /// client_mode returns the mode based on the given URL. This is used to
    /// allow clients to let the getters decide which mode to use.
    fn client_mode(&self, url: &str) -> Result<ClientMode>;

    // set_client allows a getter to know it's client
    // in order to access client's Get functions or
    // progress tracking.
    // fn set_client(&mut self, client: Client);
}
