#[derive(Copy, Clone, Debug)]
pub enum ClientMode {
    Invalid,

    // ClientModeAny downloads anything it can. In this mode, dst must
    // be a directory. If src is a file, it is saved into the directory
    // with the basename of the URL. If src is a directory or archive,
    // it is unpacked directly into dst.
    Any,

    // ClientModeFile downloads a single file. In this mode, dst must
    // be a file path (doesn't have to exist). src must point to a single
    // file. It is saved as dst.
    File,

    // ClientModeDir downloads a directory. In this mode, dst must be
    // a directory path (doesn't have to exist). src must point to an
    // archive or directory (such as in s3).
    Dir,
}
