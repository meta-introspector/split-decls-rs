macro_rules! deps {
    () => {
        FlatMapInPlace!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T > FlatMapInPlace < T > for Vec < T > { flat_map_in_place ! (Vec) ; }
    };
}

impl_50!();