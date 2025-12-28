macro_rules! deps {
    () => {
        DefId!();
    };
}

macro_rules! ItemId {
    () => {
        deps!();
        pub type ItemId = DefId ;
    };
}

ItemId!()