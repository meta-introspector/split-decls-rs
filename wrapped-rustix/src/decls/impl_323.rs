macro_rules! deps {
    () => {
        Result!();
        IncompleteArrayField!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl < T > :: core :: fmt :: Debug for IncompleteArrayField < T > { fn fmt (& self , fmt : & mut :: core :: fmt :: Formatter < '_ >) -> :: core :: fmt :: Result { fmt . write_str ("IncompleteArrayField") } }
    };
}

impl_323!();