macro_rules! deps {
    () => {
        Validity!();
        Uninit!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        unsafe impl Validity for Uninit { }
    };
}

impl_331!();