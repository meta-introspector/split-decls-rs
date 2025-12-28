macro_rules! deps {
    () => {
        IntVarValue!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        impl UnifyKey for IntVid { type Value = IntVarValue ; # [inline] fn index (& self) -> u32 { self . as_u32 () } # [inline] fn from_index (i : u32) -> IntVid { IntVid :: from_u32 (i) } fn tag () -> & 'static str { "IntVid" } }
    };
}

impl_447!()