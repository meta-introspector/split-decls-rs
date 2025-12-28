macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl PartialEq < & '_ str > for TokenText < '_ > { fn eq (& self , other : & & str) -> bool { self . as_str () == * other } }
    };
}

impl_53!();