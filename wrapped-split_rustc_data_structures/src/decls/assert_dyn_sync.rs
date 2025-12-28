macro_rules! deps {
    () => {
        DynSync!();
    };
}

macro_rules! assert_dyn_sync {
    () => {
        deps!();
        pub fn assert_dyn_sync < T : ? Sized + PointeeSized + DynSync > () { }
    };
}

assert_dyn_sync!()