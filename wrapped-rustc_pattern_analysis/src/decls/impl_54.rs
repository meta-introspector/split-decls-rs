macro_rules! deps {
    () => {
        PatCx!();
        PatOrWild!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < 'p , Cx : PatCx > fmt :: Debug for PatOrWild < 'p , Cx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { PatOrWild :: Wild => write ! (f , "_") , PatOrWild :: Pat (pat) => pat . fmt (f) , } } }
    };
}

impl_54!();