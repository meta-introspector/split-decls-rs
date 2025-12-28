macro_rules! deps {
    () => {
        DeArray!();
        DeValue!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl < 'i > core :: borrow :: Borrow < [Spanned < DeValue < 'i > >] > for DeArray < 'i > { fn borrow (& self) -> & [Spanned < DeValue < 'i > >] { & self . items [..] } }
    };
}

impl_187!();