macro_rules! deps {
    () => {
        Interner!();
        DeepRejectCtxt!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < I : Interner > DeepRejectCtxt < I , false , false > { # [doc = " Treat parameters in both the lhs and the rhs as rigid."] pub fn relate_rigid_rigid (_interner : I) -> DeepRejectCtxt < I , false , false > { DeepRejectCtxt { _interner : PhantomData } } }
    };
}

impl_45!();