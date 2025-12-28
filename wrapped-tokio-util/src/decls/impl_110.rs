macro_rules! deps {
    () => {
        MaybeDangling!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < T > MaybeDangling < T > { pub (crate) fn new (inner : T) -> Self { Self (MaybeUninit :: new (inner)) } }
    };
}

impl_110!()