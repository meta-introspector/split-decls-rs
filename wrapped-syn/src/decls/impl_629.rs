macro_rules! deps {
    () => {
        IntoPairs!();
    };
}

macro_rules! impl_629 {
    () => {
        deps!();
        impl < T , P > Clone for IntoPairs < T , P > where T : Clone , P : Clone , { fn clone (& self) -> Self { IntoPairs { inner : self . inner . clone () , last : self . last . clone () , } } }
    };
}

impl_629!();