macro_rules! deps {
    () => {
        MockTypingMode!();
        MockInferCtxt!();
        MockInferCtxtBuilder!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl MockInferCtxtBuilder { pub fn build (self , _typing_mode : MockTypingMode) -> MockInferCtxt { MockInferCtxt } }
    };
}

impl_23!()