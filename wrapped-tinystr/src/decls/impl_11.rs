macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < const N : usize > Deref for TinyAsciiStr < N > { type Target = str ; # [inline] fn deref (& self) -> & str { self . as_str () } }
    };
}

impl_11!()