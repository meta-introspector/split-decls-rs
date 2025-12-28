macro_rules! deps {
    () => {
        UnvalidatedTinyAsciiStr!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl < const N : usize > AsULE for UnvalidatedTinyAsciiStr < N > { type ULE = Self ; # [inline] fn to_unaligned (self) -> Self :: ULE { self } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { unaligned } }
    };
}

impl_89!();