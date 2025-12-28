macro_rules! deps {
    () => {
        DeArray!();
        DeValue!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < 'i > core :: ops :: DerefMut for DeArray < 'i > { # [inline] fn deref_mut (& mut self) -> & mut [Spanned < DeValue < 'i > >] { self . items . as_mut_slice () } }
    };
}

impl_184!()