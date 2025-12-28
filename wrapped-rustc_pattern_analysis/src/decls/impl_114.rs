macro_rules! deps {
    () => {
        PatCx!();
        WitnessStack!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < Cx : PatCx > Clone for WitnessStack < Cx > { fn clone (& self) -> Self { Self (self . 0 . clone ()) } }
    };
}

impl_114!();