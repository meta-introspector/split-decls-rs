macro_rules! deps {
    () => {
        AncillaryDrain!();
    };
}

macro_rules! impl_583 {
    () => {
        deps!();
        impl FusedIterator for AncillaryDrain < '_ > { }
    };
}

impl_583!()