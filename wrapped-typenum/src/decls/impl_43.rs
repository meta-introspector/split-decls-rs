macro_rules! deps {
    () => {
        NInt!();
        Unsigned!();
        NonZero!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < U : Unsigned + NonZero > NInt < U > { # [doc = " Instantiates a singleton representing this strictly negative integer."] # [inline] pub fn new () -> NInt < U > { NInt :: default () } }
    };
}

impl_43!()