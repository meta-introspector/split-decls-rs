macro_rules! deps {
    () => {
        MapDeserializer!();
        Table!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl MapDeserializer { fn new (map : Table) -> Self { Self { iter : map . into_iter () , value : None , } } }
    };
}

impl_92!()