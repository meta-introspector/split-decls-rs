macro_rules! deps {
    () => {
        Pod!();
        Zip64CDELocatorBlock!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        unsafe impl Pod for Zip64CDELocatorBlock { }
    };
}

impl_165!();