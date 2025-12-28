macro_rules! deps {
    () => {
        StreamSafe!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < I : Iterator < Item = char > + FusedIterator > FusedIterator for StreamSafe < I > { }
    };
}

impl_87!()