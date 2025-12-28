macro_rules! deps {
    () => {
        OutlivesPredicate!();
        Region!();
    };
}

macro_rules! RegionOutlivesPredicate {
    () => {
        deps!();
        pub type RegionOutlivesPredicate = OutlivesPredicate < Region , Region > ;
    };
}

RegionOutlivesPredicate!()