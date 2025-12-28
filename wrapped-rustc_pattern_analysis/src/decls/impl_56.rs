macro_rules! deps {
    () => {
        PatCx!();
        WitnessPat!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < Cx : PatCx > Clone for WitnessPat < Cx > { fn clone (& self) -> Self { Self { ctor : self . ctor . clone () , fields : self . fields . clone () , ty : self . ty . clone () } } }
    };
}

impl_56!()