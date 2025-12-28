macro_rules! deps {
    () => {
        DeArray!();
        DeValue!();
        IntoIter!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < 'i > IntoIterator for DeArray < 'i > { type Item = Spanned < DeValue < 'i > > ; type IntoIter = alloc :: vec :: IntoIter < Spanned < DeValue < 'i > > > ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . items . into_iter () } }
    };
}

impl_191!();