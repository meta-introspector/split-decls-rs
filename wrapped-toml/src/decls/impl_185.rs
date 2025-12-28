macro_rules! deps {
    () => {
        DeValue!();
        DeArray!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < 'i > AsRef < [Spanned < DeValue < 'i > >] > for DeArray < 'i > { fn as_ref (& self) -> & [Spanned < DeValue < 'i > >] { & self . items } }
    };
}

impl_185!()