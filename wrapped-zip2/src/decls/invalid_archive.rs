macro_rules! deps {
    () => {
        ZipError!();
    };
}

macro_rules! invalid_archive {
    () => {
        deps!();
        pub (crate) fn invalid_archive < M : Into < Cow < 'static , str > > > (message : M) -> ZipError { ZipError :: InvalidArchive (message . into ()) }
    };
}

invalid_archive!();