macro_rules! deps {
    () => {
        Item!();
        IntoIter!();
    };
}

macro_rules! ArraySeqAccess {
    () => {
        deps!();
        pub (crate) struct ArraySeqAccess { iter : std :: vec :: IntoIter < crate :: Item > , }
    };
}

ArraySeqAccess!()