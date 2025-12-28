macro_rules! deps {
    () => {
        AncillaryIter!();
    };
}

macro_rules! impl_596 {
    () => {
        deps!();
        impl < T > ExactSizeIterator for AncillaryIter < '_ , T > { fn len (& self) -> usize { self . data . len () / size_of :: < T > () } }
    };
}

impl_596!();