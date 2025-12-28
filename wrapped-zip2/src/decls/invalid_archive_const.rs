macro_rules! deps {
    () => {
        ZipError!();
    };
}

macro_rules! invalid_archive_const {
    () => {
        deps!();
        pub (crate) const fn invalid_archive_const (message : & 'static str) -> ZipError { ZipError :: InvalidArchive (Cow :: Borrowed (message)) }
    };
}

invalid_archive_const!();