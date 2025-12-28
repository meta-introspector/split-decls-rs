macro_rules! deps {
    () => {
        TinyAsciiStr!();
        UnvalidatedTinyAsciiStr!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < const N : usize > TinyAsciiStr < N > { # [inline] pub const fn to_unvalidated (self) -> UnvalidatedTinyAsciiStr < N > { UnvalidatedTinyAsciiStr (* self . all_bytes ()) } }
    };
}

impl_34!()