macro_rules! deps {
    () => {
        DefId!();
        LocalDefId!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl LocalDefId { pub fn to_def_id (self) -> DefId { DefId } }
    };
}

impl_4!();