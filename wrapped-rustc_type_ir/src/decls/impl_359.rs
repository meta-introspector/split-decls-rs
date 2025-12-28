macro_rules! deps {
    () => {
        PredicatePolarity!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl fmt :: Display for PredicatePolarity { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Positive => f . write_str ("positive") , Self :: Negative => f . write_str ("negative") , } } }
    };
}

impl_359!()