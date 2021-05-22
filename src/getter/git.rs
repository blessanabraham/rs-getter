use super::Getter;
use crate::{ClientMode, Result};
use async_trait::async_trait;
use futures_util::Stream;

#[derive(Copy, Clone, Debug)]
pub(crate) struct GitGetter;

#[async_trait]
impl Getter for GitGetter {
    async fn get(&self, _url: &str) -> Result<&dyn Stream<Item = &[u8]>> {
        todo!()
    }

    async fn get_file(&self, _url: &str) -> Result<&dyn Stream<Item = &[u8]>> {
        todo!()
    }

    fn client_mode(&self, _url: &str) -> Result<ClientMode> {
        todo!()
    }
}
