macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < T > Clone for Iter < '_ , T > { fn clone (& self) -> Self { Self { entries : self . entries . clone () , len : self . len , } } }
    };
}

impl_20!();