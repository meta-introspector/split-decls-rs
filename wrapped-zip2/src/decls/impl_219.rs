macro_rules! deps {
    () => {
        Pod!();
        ZipDataDescriptorBlock!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        unsafe impl Pod for ZipDataDescriptorBlock { }
    };
}

impl_219!()