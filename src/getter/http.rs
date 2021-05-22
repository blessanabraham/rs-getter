use super::Getter;
use crate::ClientMode;
use crate::{error, Result};
use async_trait::async_trait;
use core::str::FromStr;
use futures_util::Stream;

#[derive(Copy, Clone, Debug)]
pub(crate) struct HttpGetter;

impl HttpGetter {}

#[async_trait]
impl Getter for HttpGetter {
    #[cfg(not(target_arch = "wasm32"))]
    async fn get(&self, url: &str) -> Result<&dyn Stream<Item = &[u8]>> {
        use hyper::{Client, Uri};

        let uri = Uri::from_str(url).map_err(error::getter)?;
        let client = Client::new();
        let resp = client.get(uri).await.map_err(error::getter)?;

        if resp.status().as_u16() < 200 || resp.status().as_u16() >= 300 {
            return Err(error::getter(format!(
                "bad response code: {}",
                resp.status()
            )));
        }

        // Ok(resp.body())
        todo!()
    }

    #[cfg(target_arch = "wasm32")]
    async fn get(&self, url: &str) -> Result<&dyn Stream<Item = &[u8]>> {
        todo!()
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn get_file(&self, _url: &str) -> Result<&dyn Stream<Item = &[u8]>> {
        todo!()
    }

    #[cfg(target_arch = "wasm32")]
    async fn get_file(&self, url: &str) -> Result<&dyn Stream<Item = &[u8]>> {
        todo!()
    }

    fn client_mode(&self, url: &str) -> Result<ClientMode> {
        if url.ends_with('/') {
            Ok(ClientMode::Dir)
        } else {
            Ok(ClientMode::File)
        }
    }
}
