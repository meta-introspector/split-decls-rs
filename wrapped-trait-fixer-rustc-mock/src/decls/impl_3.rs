macro_rules! deps {
    () => {
        LocalDefId!();
        DefId!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl DefId { pub fn as_local (self) -> Option < LocalDefId > { Some (LocalDefId) } pub fn to_def_id (self) -> DefId { self } }
    };
}

impl_3!();