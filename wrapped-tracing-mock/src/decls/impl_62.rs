macro_rules! deps {
    () => {
        ExpectedId!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl fmt :: Debug for ExpectedId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("ExpectedId") . field (& self . inner) . finish () } }
    };
}

impl_62!()