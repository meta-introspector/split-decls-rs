macro_rules! deps {
    () => {
        Index!();
        Result!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl fmt :: Debug for Index { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Index") . field ("value" , & self . 0) . finish () } }
    };
}

impl_34!();