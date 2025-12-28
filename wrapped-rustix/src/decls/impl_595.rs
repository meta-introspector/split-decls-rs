macro_rules! deps {
    () => {
        AncillaryIter!();
    };
}

macro_rules! impl_595 {
    () => {
        deps!();
        impl < T > FusedIterator for AncillaryIter < '_ , T > { }
    };
}

impl_595!()