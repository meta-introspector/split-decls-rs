macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < const N : usize > Borrow < str > for TinyAsciiStr < N > { # [inline] fn borrow (& self) -> & str { self . as_str () } }
    };
}

impl_12!();