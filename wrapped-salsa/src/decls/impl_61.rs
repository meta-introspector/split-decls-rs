macro_rules! deps {
    () => {
        CycleHeadsIterator!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl FusedIterator for CycleHeadsIterator < '_ > { }
    };
}

impl_61!()