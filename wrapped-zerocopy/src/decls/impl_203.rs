macro_rules! deps {
    () => {
        SizeError!();
        KnownLayout!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        # [cfg (any (zerocopy_core_error_1_81_0 , feature = "std" , test))] # [cfg_attr (doc_cfg , doc (cfg (all (rust = "1.81.0" , feature = "std"))))] impl < Src , Dst : ? Sized > Error for SizeError < Src , Dst > where Src : Deref , Dst : KnownLayout , { }
    };
}

impl_203!();