macro_rules! deps {
    () => {
        ZipDataDescriptorBlock!();
        Pod!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        unsafe impl Pod for ZipDataDescriptorBlock { }
    };
}

impl_219!();