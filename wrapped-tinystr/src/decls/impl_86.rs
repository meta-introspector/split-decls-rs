macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl < const N : usize > AsULE for TinyAsciiStr < N > { type ULE = Self ; # [inline] fn to_unaligned (self) -> Self :: ULE { self } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { unaligned } }
    };
}

impl_86!();