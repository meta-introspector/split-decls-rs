macro_rules! deps {
    () => {
        FloatVarValue!();
    };
}

macro_rules! impl_449 {
    () => {
        deps!();
        impl UnifyKey for FloatVid { type Value = FloatVarValue ; # [inline] fn index (& self) -> u32 { self . as_u32 () } # [inline] fn from_index (i : u32) -> FloatVid { FloatVid :: from_u32 (i) } fn tag () -> & 'static str { "FloatVid" } }
    };
}

impl_449!();