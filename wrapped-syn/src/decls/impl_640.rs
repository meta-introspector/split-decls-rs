macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_640 {
    () => {
        deps!();
        impl < 'a , T > Clone for Iter < 'a , T > { fn clone (& self) -> Self { Iter { inner : self . inner . clone_box () , } } }
    };
}

impl_640!()