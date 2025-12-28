macro_rules! deps {
    () => {
        LocalDefId!();
        DefId!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl LocalDefId { # [inline] pub fn to_def_id (self) -> DefId { DefId { krate : LOCAL_CRATE , index : self . local_def_index } } # [inline] pub fn is_top_level_module (self) -> bool { self == CRATE_DEF_ID } }
    };
}

impl_118!()