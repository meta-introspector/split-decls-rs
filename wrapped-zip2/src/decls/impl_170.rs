macro_rules! deps {
    () => {
        Zip64CDEBlock!();
        Pod!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        unsafe impl Pod for Zip64CDEBlock { }
    };
}

impl_170!()