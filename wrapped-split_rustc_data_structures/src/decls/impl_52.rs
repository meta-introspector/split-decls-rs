macro_rules! deps {
    () => {
        FlatMapInPlace!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < T > FlatMapInPlace < T > for ThinVec < T > { flat_map_in_place ! (ThinVec) ; }
    };
}

impl_52!();