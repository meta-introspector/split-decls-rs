macro_rules! deps {
    () => {
        Zip64DataDescriptorBlock!();
        Pod!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        unsafe impl Pod for Zip64DataDescriptorBlock { }
    };
}

impl_222!();