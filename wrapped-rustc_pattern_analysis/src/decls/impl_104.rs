macro_rules! deps {
    () => {
        PatCx!();
        PatStack!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > Clone for PatStack < 'p , Cx > { fn clone (& self) -> Self { Self { pats : self . pats . clone () , relevant : self . relevant } } }
    };
}

impl_104!()