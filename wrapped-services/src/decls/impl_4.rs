macro_rules! deps {
    () => {
        ExtendedCommand!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        unsafe impl Sync for ExtendedCommand { }
    };
}

impl_4!()