macro_rules! deps {
    () => {
        WorkProduct!();
        WorkProductId!();
    };
}

macro_rules! WorkProductMap {
    () => {
        deps!();
        pub type WorkProductMap = UnordMap < WorkProductId , WorkProduct > ;
    };
}

WorkProductMap!();