macro_rules! deps {
    () => {
        DefId!();
        OwnerId!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl OwnerId { pub const DUMMY : Self = Self ; pub fn to_def_id (self) -> DefId { DefId } }
    };
}

impl_5!()