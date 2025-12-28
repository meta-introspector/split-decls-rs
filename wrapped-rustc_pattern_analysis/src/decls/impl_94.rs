macro_rules! deps {
    () => {
        PatCx!();
        PlaceCtxt!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < 'a , Cx : PatCx > Clone for PlaceCtxt < 'a , Cx > { fn clone (& self) -> Self { Self { cx : self . cx , ty : self . ty } } }
    };
}

impl_94!();