macro_rules! deps {
    () => {
        TransitiveRelation!();
        Frozen!();
    };
}

macro_rules! impl_618 {
    () => {
        deps!();
        impl < T : Clone > Clone for TransitiveRelation < T > { fn clone (& self) -> Self { TransitiveRelation { builder : Frozen :: freeze (self . builder . deref () . clone ()) , closure : Frozen :: freeze (self . closure . deref () . clone ()) , } } }
    };
}

impl_618!()