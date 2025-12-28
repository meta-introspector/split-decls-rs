macro_rules! deps {
    () => {
        TokenText!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl PartialEq < String > for TokenText < '_ > { fn eq (& self , other : & String) -> bool { self . as_str () == other . as_str () } }
    };
}

impl_55!();