macro_rules! deps {
    () => {
        LocalDefId!();
        DefId!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl From < LocalDefId > for DefId { fn from (local : LocalDefId) -> DefId { local . to_def_id () } }
    };
}

impl_108!();