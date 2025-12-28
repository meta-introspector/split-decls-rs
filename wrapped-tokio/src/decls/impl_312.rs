macro_rules! deps {
    () => {
        Pointers!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl < T > fmt :: Debug for Pointers < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let prev = self . get_prev () ; let next = self . get_next () ; f . debug_struct ("Pointers") . field ("prev" , & prev) . field ("next" , & next) . finish () } }
    };
}

impl_312!()