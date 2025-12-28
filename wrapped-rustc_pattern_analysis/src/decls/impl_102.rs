macro_rules! deps {
    () => {
        PatCx!();
        PlaceInfo!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < Cx : PatCx > Clone for PlaceInfo < Cx > { fn clone (& self) -> Self { Self { ty : self . ty . clone () , private_uninhabited : self . private_uninhabited , validity : self . validity , is_scrutinee : self . is_scrutinee , } } }
    };
}

impl_102!()