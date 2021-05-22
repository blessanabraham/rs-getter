use crate::ClientMode;

/// Client is a client for downloading things.
///
/// Top-level functions such as Get are shortcuts for interacting with a client.
/// Using a client directly allows more fine-grained control over how downloading
/// is done, as well as customizing the protocols supported.
#[derive(Debug)]
pub(crate) struct Client {
    /// Src is the source URL to get.
    src: String,

    /// Mode is the method of download the client will use. See [ClientMode]
    /// for documentation.
    mode: ClientMode,
    // Decompressors is the map of decompressors supported by this client.
    // If this is nil, then the default value is the Decompressors global.
    // Decompressors map[string]Decompressor
    // Getters is the map of protocols supported by this client. If this
    // is nil, then the default Getters variable will be used.
    // getters: BTreeMap<&'static str, Box<dyn Getter<Stream = dyn StreamExt<Item = dyn Any>>>>,
}
