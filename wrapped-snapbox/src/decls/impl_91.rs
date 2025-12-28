macro_rules! deps {
    () => {
        DataSource!();
        Inline!();
        Result!();
        DataSourceInner!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl std :: fmt :: Display for DataSource { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self . inner { DataSourceInner :: Path (value) => crate :: dir :: display_relpath (value) . fmt (f) , DataSourceInner :: Inline (value) => value . fmt (f) , } } }
    };
}

impl_91!();