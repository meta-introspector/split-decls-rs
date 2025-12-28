macro_rules! deps {
    () => {
        InlinedName!();
    };
}

macro_rules! impl_674 {
    () => {
        deps!();
        impl core :: borrow :: Borrow < str > for InlinedName { fn borrow (& self) -> & str { self . as_ref () } }
    };
}

impl_674!();