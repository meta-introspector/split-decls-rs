macro_rules! deps {
    () => {
        DeValue!();
        DeArray!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < 'i > core :: borrow :: BorrowMut < [Spanned < DeValue < 'i > >] > for DeArray < 'i > { fn borrow_mut (& mut self) -> & mut [Spanned < DeValue < 'i > >] { & mut self . items [..] } }
    };
}

impl_188!()