macro_rules! deps {
    () => {
        MockTypingMode!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Default for MockTypingMode { fn default () -> Self { MockTypingMode :: NonBodyAnalysis } }
    };
}

impl_25!()