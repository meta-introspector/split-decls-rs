macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl PartialEq < SmolStr > for SmolStr { fn eq (& self , other : & SmolStr) -> bool { self . 0 . ptr_eq (& other . 0) || self . as_str () == other . as_str () } }
    };
}

impl_7!();