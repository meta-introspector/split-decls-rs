macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! START {
    () => {
        deps!();
        # [doc = " Value of the initial revision, as a u64. We don't use 0"] # [doc = " because we want to use a `NonZeroUsize`."] const START : usize = 1 ;
    };
}

START!()