# rs-getter

rs-getter is a partial port of [go-getter](https://github.com/hashicorp/go-getter).
rs-getter is a library for Rust for downloading files or directories
from various sources using a URL as the primary form of input.

The power of this library is being flexible in being able to download
from a number of different sources (file paths, Git, HTTP, Mercurial, etc.)
using a single string as input. This removes the burden of knowing how to
download from a variety of sources from the implementer.

The concept of a _detector_ automatically turns invalid URLs into proper
URLs. For example: "github.com/blessanabraham/rs-getter" would turn into a
Git URL. Or "./foo" would turn into a file URL. These are extensible.

This library is used by the [Rust port of USD](https://github.com/blessanabraham/usd-rs) for
downloading plugins and other dependencies.
