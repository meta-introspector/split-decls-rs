macro_rules! deps {
    () => {
        DeValue!();
        DeArray!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < 'i > core :: ops :: Deref for DeArray < 'i > { type Target = [Spanned < DeValue < 'i > >] ; # [inline] fn deref (& self) -> & [Spanned < DeValue < 'i > >] { self . items . as_slice () } }
    };
}

impl_183!();