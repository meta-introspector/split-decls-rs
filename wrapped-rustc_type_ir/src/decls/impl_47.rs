macro_rules! deps {
    () => {
        Interner!();
        DeepRejectCtxt!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < I : Interner > DeepRejectCtxt < I , false , true > { # [doc = " Treat parameters in the lhs as rigid, and in rhs as infer vars."] pub fn relate_rigid_infer (_interner : I) -> DeepRejectCtxt < I , false , true > { DeepRejectCtxt { _interner : PhantomData } } }
    };
}

impl_47!()