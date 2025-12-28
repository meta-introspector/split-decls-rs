macro_rules! deps {
    () => {
        Recompositions!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > + FusedIterator > FusedIterator for Recompositions < I > { }
    };
}

impl_73!();