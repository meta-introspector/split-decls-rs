macro_rules! deps {
    () => {
        Region!();
        OutlivesPredicate!();
    };
}

macro_rules! RegionOutlivesPredicate {
    () => {
        deps!();
        pub type RegionOutlivesPredicate = OutlivesPredicate < Region , Region > ;
    };
}

RegionOutlivesPredicate!();