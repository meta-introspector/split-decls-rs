macro_rules! deps {
    () => {
        UTerm!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        impl UTerm { # [doc = " Instantiates a singleton representing this unsigned integer."] # [inline] pub fn new () -> UTerm { UTerm } }
    };
}

impl_339!();