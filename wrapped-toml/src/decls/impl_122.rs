macro_rules! deps {
    () => {
        ArraySeqAccess!();
        DeArray!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < 'i > ArraySeqAccess < 'i > { pub (crate) fn new (input : DeArray < 'i >) -> Self { Self { iter : input . into_iter () , } } }
    };
}

impl_122!();