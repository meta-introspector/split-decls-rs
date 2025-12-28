macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl Eq for Index { }
    };
}

impl_37!()