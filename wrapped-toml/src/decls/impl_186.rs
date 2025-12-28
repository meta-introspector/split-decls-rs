macro_rules! deps {
    () => {
        DeValue!();
        DeArray!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < 'i > AsMut < [Spanned < DeValue < 'i > >] > for DeArray < 'i > { fn as_mut (& mut self) -> & mut [Spanned < DeValue < 'i > >] { & mut self . items } }
    };
}

impl_186!();