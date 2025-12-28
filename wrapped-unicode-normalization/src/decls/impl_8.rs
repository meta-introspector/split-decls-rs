macro_rules! deps {
    () => {
        Decompositions!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > + FusedIterator > FusedIterator for Decompositions < I > { }
    };
}

impl_8!();