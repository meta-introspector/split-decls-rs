macro_rules! deps {
    () => {
        AsInitialized!();
        Validity!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        unsafe impl Validity for AsInitialized { }
    };
}

impl_333!();