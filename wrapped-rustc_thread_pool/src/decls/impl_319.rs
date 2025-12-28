macro_rules! deps {
    () => {
        Tlv!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        unsafe impl Sync for Tlv { }
    };
}

impl_319!()