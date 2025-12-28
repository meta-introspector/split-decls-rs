macro_rules! deps {
    () => {
        ZipResult!();
        ZipError!();
    };
}

macro_rules! unsupported_zip_error {
    () => {
        deps!();
        const fn unsupported_zip_error < T > (detail : & 'static str) -> ZipResult < T > { Err (ZipError :: UnsupportedArchive (detail)) }
    };
}

unsupported_zip_error!()