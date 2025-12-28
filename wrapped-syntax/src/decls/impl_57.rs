macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl PartialEq for TokenText < '_ > { fn eq (& self , other : & TokenText < '_ >) -> bool { self . as_str () == other . as_str () } }
    };
}

impl_57!();