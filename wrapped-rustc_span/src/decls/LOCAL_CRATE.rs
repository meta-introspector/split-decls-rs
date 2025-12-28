macro_rules! deps {
    () => {
        DefId!();
    };
}

macro_rules! LOCAL_CRATE {
    () => {
        deps!();
        # [doc = " Item definitions in the currently-compiled crate would have the `CrateNum`"] # [doc = " `LOCAL_CRATE` in their `DefId`."] pub const LOCAL_CRATE : CrateNum = CrateNum :: ZERO ;
    };
}

LOCAL_CRATE!();