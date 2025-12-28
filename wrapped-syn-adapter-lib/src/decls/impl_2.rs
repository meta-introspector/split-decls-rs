macro_rules! deps {
    () => {
        MockSynAdapter!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl MockSynAdapter { pub fn new () -> Self { MockSynAdapter } }
    };
}

impl_2!();