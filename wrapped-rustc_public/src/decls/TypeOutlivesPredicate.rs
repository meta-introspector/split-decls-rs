macro_rules! deps {
    () => {
        OutlivesPredicate!();
        Ty!();
        Region!();
    };
}

macro_rules! TypeOutlivesPredicate {
    () => {
        deps!();
        pub type TypeOutlivesPredicate = OutlivesPredicate < Ty , Region > ;
    };
}

TypeOutlivesPredicate!();