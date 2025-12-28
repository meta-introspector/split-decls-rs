macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl PartialEq for Identifier { fn eq (& self , rhs : & Self) -> bool { if self . ptr_eq (rhs) { true } else if self . is_empty_or_inline () || rhs . is_empty_or_inline () { false } else { unsafe { ptr_as_str (& self . head) == ptr_as_str (& rhs . head) } } } }
    };
}

impl_37!()