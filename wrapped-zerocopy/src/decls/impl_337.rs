macro_rules! deps {
    () => {
        Validity!();
        Valid!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        unsafe impl Validity for Valid { }
    };
}

impl_337!();