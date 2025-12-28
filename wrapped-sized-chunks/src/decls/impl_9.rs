macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'a , A , T > FusedIterator for Drain < 'a , A , T > { }
    };
}

impl_9!()