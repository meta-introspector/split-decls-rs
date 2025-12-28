macro_rules! deps {
    () => {
        GrowingHashmapChar!();
    };
}

macro_rules! HybridGrowingHashmapChar {
    () => {
        deps!();
        struct HybridGrowingHashmapChar < ValueType > { map : GrowingHashmapChar < ValueType > , extended_ascii : [ValueType ; 256] , }
    };
}

HybridGrowingHashmapChar!()