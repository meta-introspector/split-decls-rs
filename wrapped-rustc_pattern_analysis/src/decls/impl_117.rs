macro_rules! deps {
    () => {
        PatCx!();
        WitnessMatrix!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < Cx : PatCx > Clone for WitnessMatrix < Cx > { fn clone (& self) -> Self { Self (self . 0 . clone ()) } }
    };
}

impl_117!()