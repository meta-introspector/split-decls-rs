macro_rules! deps {
    () => {
        Item!();
        ArraySeqAccess!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl ArraySeqAccess { pub (crate) fn new (input : Vec < crate :: Item >) -> Self { Self { iter : input . into_iter () , } } }
    };
}

impl_282!();