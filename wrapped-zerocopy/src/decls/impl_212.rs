macro_rules! deps {
    () => {
        TryFromBytes!();
        ValidityError!();
        KnownLayout!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        # [cfg (any (zerocopy_core_error_1_81_0 , feature = "std" , test))] # [cfg_attr (doc_cfg , doc (cfg (all (rust = "1.81.0" , feature = "std"))))] impl < Src , Dst : ? Sized > Error for ValidityError < Src , Dst > where Dst : KnownLayout + TryFromBytes { }
    };
}

impl_212!()