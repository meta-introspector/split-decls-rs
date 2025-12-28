macro_rules! deps {
    () => {
        IntoIter!();
        DeValue!();
    };
}

macro_rules! ArraySeqAccess {
    () => {
        deps!();
        pub (crate) struct ArraySeqAccess < 'i > { iter : alloc :: vec :: IntoIter < Spanned < DeValue < 'i > > > , }
    };
}

ArraySeqAccess!()