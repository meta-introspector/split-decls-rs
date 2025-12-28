macro_rules! deps {
    () => {
        ZipCentralEntryBlock!();
        Pod!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        unsafe impl Pod for ZipCentralEntryBlock { }
    };
}

impl_210!()