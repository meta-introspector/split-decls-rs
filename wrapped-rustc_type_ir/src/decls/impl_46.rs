macro_rules! deps {
    () => {
        Interner!();
        DeepRejectCtxt!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < I : Interner > DeepRejectCtxt < I , true , true > { # [doc = " Treat parameters in both the lhs and the rhs as infer vars."] pub fn relate_infer_infer (_interner : I) -> DeepRejectCtxt < I , true , true > { DeepRejectCtxt { _interner : PhantomData } } }
    };
}

impl_46!();