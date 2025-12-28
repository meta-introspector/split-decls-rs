macro_rules! deps {
    () => {
        QueryStackDeferred!();
    };
}

macro_rules! impl_210 {
    () => {
        deps!();
        impl < 'tcx > Debug for QueryStackDeferred < 'tcx > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str ("QueryStackDeferred") } }
    };
}

impl_210!();