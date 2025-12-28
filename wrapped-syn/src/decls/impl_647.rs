macro_rules! deps {
    () => {
        PrivateIter!();
    };
}

macro_rules! impl_647 {
    () => {
        deps!();
        impl < 'a , T , P > Clone for PrivateIter < 'a , T , P > { fn clone (& self) -> Self { PrivateIter { inner : self . inner . clone () , last : self . last . clone () , } } }
    };
}

impl_647!();