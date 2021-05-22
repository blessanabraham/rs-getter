#![cfg_attr(target_arch = "wasm32", allow(unused))]
use std::error::Error as StdError;
use std::fmt;
use url::Url;

pub(crate) type Result<T> = core::result::Result<T, Error>;

// pub(crate) trait StdError: fmt::Display + fmt::Debug + Send + Sync {
//     fn source(&self) -> Option<&(dyn StdError + 'static)>;
// }

pub(crate) type BoxError = Box<dyn StdError>;

#[derive(Debug)]
pub(crate) enum Kind {
    Detector,
    Getter,
}

struct Inner {
    kind: Kind,
    source: Option<BoxError>,
    url: Option<Url>,
}

/// The Errors that may occur when processing.
pub struct Error {
    inner: Box<Inner>,
}

impl Error {
    pub(crate) fn new<E>(kind: Kind, source: Option<E>) -> Error
    where
        E: Into<BoxError>,
    {
        Error {
            inner: Box::new(Inner {
                kind,
                source: source.map(Into::into),
                url: None,
            }),
        }
    }

    pub fn url(&self) -> Option<&Url> {
        self.inner.url.as_ref()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("getter::Error");

        builder.field("kind", &self.inner.kind);

        if let Some(ref url) = self.inner.url {
            builder.field("url", url);
        }
        if let Some(ref source) = self.inner.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct ForUrl<'a>(Option<&'a Url>);

        impl fmt::Display for ForUrl<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if let Some(url) = self.0 {
                    write!(f, " for url ({})", url.as_str())
                } else {
                    Ok(())
                }
            }
        }

        match self.inner.kind {
            Kind::Detector => f.write_str("detector error")?,
            Kind::Getter => f.write_str("getter error")?,
        };

        ForUrl(self.inner.url.as_ref()).fmt(f)?;

        if let Some(ref e) = self.inner.source {
            write!(f, ": {}", e)?;
        }

        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner.source.as_ref().map(|e| &**e as _)
    }
}

#[cfg(target_arch = "wasm32")]
impl From<crate::error::Error> for wasm_bindgen::JsValue {
    fn from(err: Error) -> wasm_bindgen::JsValue {
        js_sys::Error::from(err).into()
    }
}

#[cfg(target_arch = "wasm32")]
impl From<crate::error::Error> for js_sys::Error {
    fn from(err: Error) -> js_sys::Error {
        js_sys::Error::new(&format!("{}", err))
    }
}

// constructors

pub(crate) fn detector<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Detector, Some(e))
}

pub(crate) fn getter<E: Into<BoxError>>(e: E) -> Error {
    Error::new(Kind::Getter, Some(e))
}
