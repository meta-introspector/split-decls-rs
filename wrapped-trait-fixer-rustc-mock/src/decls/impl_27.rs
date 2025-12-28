macro_rules! deps {
    () => {
        MockInferCtxt!();
        MockInferCtxtAt!();
        MockObligationCause!();
        MockParamEnv!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl MockInferCtxt { pub fn probe (self , f : impl FnOnce (& MockInferCtxt) -> bool) -> bool { f (& self) } pub fn at (self , _cause : & MockObligationCause , _param_env : MockParamEnv) -> MockInferCtxtAt { MockInferCtxtAt } }
    };
}

impl_27!()