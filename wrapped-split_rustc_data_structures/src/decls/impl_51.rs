macro_rules! deps {
    () => {
        FlatMapInPlace!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < T , A : Array < Item = T > > FlatMapInPlace < T > for SmallVec < A > { flat_map_in_place ! (SmallVec where T : Array) ; }
    };
}

impl_51!()