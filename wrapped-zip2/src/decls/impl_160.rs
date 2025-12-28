macro_rules! deps {
    () => {
        Pod!();
        Zip32CDEBlock!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        unsafe impl Pod for Zip32CDEBlock { }
    };
}

impl_160!();