macro_rules! deps {
    () => {
        GrowingHashmapChar!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < ValueType > Default for GrowingHashmapChar < ValueType > where ValueType : Default + Clone + Eq , { fn default () -> Self { Self { used : 0 , fill : 0 , mask : - 1 , map : None , } } }
    };
}

impl_22!()