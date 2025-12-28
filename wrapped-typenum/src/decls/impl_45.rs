macro_rules! deps {
    () => {
        Z0!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl Z0 { # [doc = " Instantiates a singleton representing the integer 0."] # [inline] pub fn new () -> Z0 { Z0 } }
    };
}

impl_45!();