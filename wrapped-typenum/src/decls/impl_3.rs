macro_rules! deps {
    () => {
        B1!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl B1 { # [doc = " Instantiates a singleton representing this bit."] # [inline] pub fn new () -> B1 { B1 } }
    };
}

impl_3!();