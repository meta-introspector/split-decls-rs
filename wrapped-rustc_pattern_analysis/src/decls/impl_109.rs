macro_rules! deps {
    () => {
        PatCx!();
        MatrixRow!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > fmt :: Debug for MatrixRow < 'p , Cx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . pats . fmt (f) } }
    };
}

impl_109!();