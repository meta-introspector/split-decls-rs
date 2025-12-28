macro_rules! deps {
    () => {
        Pairs!();
    };
}

macro_rules! impl_620 {
    () => {
        deps!();
        impl < 'a , T , P > Clone for Pairs < 'a , T , P > { fn clone (& self) -> Self { Pairs { inner : self . inner . clone () , last : self . last . clone () , } } }
    };
}

impl_620!();