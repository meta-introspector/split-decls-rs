macro_rules! deps {
    () => {
        WorkProductId!();
        WorkProduct!();
    };
}

macro_rules! WorkProductMap {
    () => {
        deps!();
        pub type WorkProductMap = UnordMap < WorkProductId , WorkProduct > ;
    };
}

WorkProductMap!()