macro_rules! deps {
    () => {
        InlinedName!();
    };
}

macro_rules! impl_671 {
    () => {
        deps!();
        impl InlinedName { # [doc = " Returns the str representation of the inlined name."] pub fn as_str (& self) -> & str { self . as_ref () } # [doc = " Returns the bytes representation of the inlined name."] pub fn as_bytes (& self) -> & [u8] { self . as_ref () } }
    };
}

impl_671!()