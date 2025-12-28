macro_rules! deps {
    () => {
        MockSubsts!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl MockSubsts { pub fn empty () -> Self { MockSubsts } }
    };
}

impl_39!()