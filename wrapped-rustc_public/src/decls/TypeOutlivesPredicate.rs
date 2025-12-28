macro_rules! deps {
    () => {
        Ty!();
        Region!();
        OutlivesPredicate!();
    };
}

macro_rules! TypeOutlivesPredicate {
    () => {
        deps!();
        pub type TypeOutlivesPredicate = OutlivesPredicate < Ty , Region > ;
    };
}

TypeOutlivesPredicate!()