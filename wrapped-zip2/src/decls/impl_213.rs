macro_rules! deps {
    () => {
        ZipLocalEntryBlock!();
        Pod!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        unsafe impl Pod for ZipLocalEntryBlock { }
    };
}

impl_213!()