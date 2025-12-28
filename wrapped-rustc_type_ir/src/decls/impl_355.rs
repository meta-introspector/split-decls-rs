macro_rules! deps {
    () => {
        ImplPolarity!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl fmt :: Display for ImplPolarity { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Positive => f . write_str ("positive") , Self :: Negative => f . write_str ("negative") , Self :: Reservation => f . write_str ("reservation") , } } }
    };
}

impl_355!();