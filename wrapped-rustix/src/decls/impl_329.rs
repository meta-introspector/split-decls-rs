macro_rules! deps {
    () => {
        UnionField!();
        Result!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < T > :: core :: fmt :: Debug for UnionField < T > { fn fmt (& self , fmt : & mut :: core :: fmt :: Formatter < '_ >) -> :: core :: fmt :: Result { fmt . write_str ("UnionField") } }
    };
}

impl_329!();