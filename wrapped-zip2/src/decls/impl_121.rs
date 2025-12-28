macro_rules! deps {
    () => {
        ZipFileData!();
        HasZipMetadata!();
        ZipFileSeek!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < R > HasZipMetadata for ZipFileSeek < '_ , R > { fn get_metadata (& self) -> & ZipFileData { self . data . as_ref () } }
    };
}

impl_121!()