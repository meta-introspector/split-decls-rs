macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! impl_634 {
    () => {
        deps!();
        impl < T > Clone for IntoIter < T > where T : Clone , { fn clone (& self) -> Self { IntoIter { inner : self . inner . clone () , } } }
    };
}

impl_634!();