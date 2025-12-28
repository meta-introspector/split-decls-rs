macro_rules! deps {
    () => {
        HasZipMetadata!();
        ZipFileData!();
        ZipFile!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < R : Read > HasZipMetadata for ZipFile < '_ , R > { fn get_metadata (& self) -> & ZipFileData { self . data . as_ref () } }
    };
}

impl_117!();