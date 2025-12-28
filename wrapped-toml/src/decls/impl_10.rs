macro_rules! deps {
    () => {
        Map!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < K : Clone , V : Clone > Clone for Map < K , V > { # [inline] fn clone (& self) -> Self { Self { map : self . map . clone () , dotted : self . dotted , implicit : self . implicit , inline : self . inline , } } }
    };
}

impl_10!();