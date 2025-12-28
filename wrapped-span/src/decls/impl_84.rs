macro_rules! deps {
    () => {
        EditionedFileId!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl From < EditionedFileId > for FileId { fn from (value : EditionedFileId) -> Self { value . file_id () } }
    };
}

impl_84!()