macro_rules! deps {
    () => {
        Segment!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl Eq for Segment { }
    };
}

impl_202!()