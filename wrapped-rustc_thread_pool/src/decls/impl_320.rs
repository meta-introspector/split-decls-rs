macro_rules! deps {
    () => {
        Tlv!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        unsafe impl Send for Tlv { }
    };
}

impl_320!()