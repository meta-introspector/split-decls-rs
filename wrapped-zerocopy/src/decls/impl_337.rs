macro_rules! deps {
    () => {
        Valid!();
        Validity!();
    };
}

macro_rules! impl_337 {
    () => {
        deps!();
        unsafe impl Validity for Valid { }
    };
}

impl_337!()