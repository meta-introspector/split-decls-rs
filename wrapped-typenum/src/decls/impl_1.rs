macro_rules! deps {
    () => {
        B0!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl B0 { # [doc = " Instantiates a singleton representing this bit."] # [inline] pub fn new () -> B0 { B0 } }
    };
}

impl_1!()