macro_rules! deps {
    () => {
        Service!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < S > Clone for Service < S > where S : Clone , { fn clone (& self) -> Self { Service { span : self . span . clone () , inner : self . inner . clone () , } } }
    };
}

impl_12!();