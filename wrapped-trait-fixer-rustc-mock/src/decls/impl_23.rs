macro_rules! deps {
    () => {
        MockInferCtxtBuilder!();
        MockInferCtxt!();
        MockTypingMode!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl MockInferCtxtBuilder { pub fn build (self , _typing_mode : MockTypingMode) -> MockInferCtxt { MockInferCtxt } }
    };
}

impl_23!();