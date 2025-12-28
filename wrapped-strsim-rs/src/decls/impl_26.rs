macro_rules! deps {
    () => {
        GrowingHashmapChar!();
        HybridGrowingHashmapChar!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < ValueType > Default for HybridGrowingHashmapChar < ValueType > where ValueType : Default + Clone + Copy + Eq , { fn default () -> Self { HybridGrowingHashmapChar { map : GrowingHashmapChar :: default () , extended_ascii : [Default :: default () ; 256] , } } }
    };
}

impl_26!()