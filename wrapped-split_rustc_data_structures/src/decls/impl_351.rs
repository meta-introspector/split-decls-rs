macro_rules! deps {
    () => {
        Pu128!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl Pu128 { # [inline] pub fn get (self) -> u128 { self . 0 } }
    };
}

impl_351!()