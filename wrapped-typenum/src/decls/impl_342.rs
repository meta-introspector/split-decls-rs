macro_rules! deps {
    () => {
        Bit!();
        UInt!();
        Unsigned!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl < U : Unsigned , B : Bit > UInt < U , B > { # [doc = " Instantiates a singleton representing this unsigned integer."] # [inline] pub fn new () -> UInt < U , B > { UInt :: default () } }
    };
}

impl_342!()