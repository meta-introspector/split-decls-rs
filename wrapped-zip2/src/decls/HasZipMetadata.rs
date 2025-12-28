macro_rules! deps {
    () => {
        ZipFileData!();
    };
}

macro_rules! HasZipMetadata {
    () => {
        deps!();
        # [doc = " A trait for exposing file metadata inside the zip."] pub trait HasZipMetadata { # [doc = " Get the file metadata"] fn get_metadata (& self) -> & ZipFileData ; }
    };
}

HasZipMetadata!();