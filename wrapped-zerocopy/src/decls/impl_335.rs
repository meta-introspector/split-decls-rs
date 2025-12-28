macro_rules! deps {
    () => {
        Initialized!();
        Validity!();
    };
}

macro_rules! impl_335 {
    () => {
        deps!();
        unsafe impl Validity for Initialized { }
    };
}

impl_335!()