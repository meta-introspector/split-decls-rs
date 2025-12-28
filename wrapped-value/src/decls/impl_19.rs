macro_rules! deps {
    () => {
        MapDeserializer!();
        Name!();
        ConstValue!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl MapDeserializer { # [inline] fn new (map : IndexMap < Name , ConstValue >) -> Self { MapDeserializer { iter : map . into_iter () , value : None , } } }
    };
}

impl_19!()