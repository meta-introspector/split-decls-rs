macro_rules! deps {
    () => {
        Members!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl < 'a > Clone for Members < 'a > { fn clone (& self) -> Self { Members { fields : self . fields . clone () , index : self . index , } } }
    };
}

impl_147!();