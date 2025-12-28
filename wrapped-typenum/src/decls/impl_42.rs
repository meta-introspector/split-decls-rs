macro_rules! deps {
    () => {
        NonZero!();
        Unsigned!();
        PInt!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero > PInt < U > { # [doc = " Instantiates a singleton representing this strictly positive integer."] # [inline] pub fn new () -> PInt < U > { PInt :: default () } }
    };
}

impl_42!();