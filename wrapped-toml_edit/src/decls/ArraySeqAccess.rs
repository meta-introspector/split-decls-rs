macro_rules! deps {
    () => {
        IntoIter!();
        Item!();
    };
}

macro_rules! ArraySeqAccess {
    () => {
        deps!();
        pub (crate) struct ArraySeqAccess { iter : std :: vec :: IntoIter < crate :: Item > , }
    };
}

ArraySeqAccess!();