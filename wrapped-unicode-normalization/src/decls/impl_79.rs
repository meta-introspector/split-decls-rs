macro_rules! deps {
    () => {
        Replacements!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > + FusedIterator > FusedIterator for Replacements < I > { }
    };
}

impl_79!();