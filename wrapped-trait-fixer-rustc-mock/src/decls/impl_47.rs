macro_rules! deps {
    () => {
        MockBinder!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl MockBinder { pub fn dummy < T > (_value : T) -> MockBinder { MockBinder } }
    };
}

impl_47!();