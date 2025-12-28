macro_rules! deps {
    () => {
        Field!();
        Iter!();
        FieldSet!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl IntoIterator for & FieldSet { type IntoIter = Iter ; type Item = Field ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_175!()