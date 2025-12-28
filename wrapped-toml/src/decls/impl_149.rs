macro_rules! deps {
    () => {
        ValueDeserializer!();
        DeValue!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < 'i > From < Spanned < DeValue < 'i > > > for ValueDeserializer < 'i > { fn from (root : Spanned < DeValue < 'i > >) -> Self { let span = root . span () ; let root = root . into_inner () ; Self :: with_parts (root , span) } }
    };
}

impl_149!();