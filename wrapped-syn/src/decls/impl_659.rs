macro_rules! deps {
    () => {
        PrivateIterMut!();
    };
}

macro_rules! impl_659 {
    () => {
        deps!();
        impl < 'a , T , P > ExactSizeIterator for PrivateIterMut < 'a , T , P > { fn len (& self) -> usize { self . inner . len () + self . last . len () } }
    };
}

impl_659!()