macro_rules! deps {
    () => {
        MockSubsts!();
        MockEarlyBinder!();
        MockTy!();
        TyCtxt!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl < 'tcx > MockEarlyBinder < 'tcx > { pub fn instantiate (self , _tcx : TyCtxt < 'tcx > , _substs : MockSubsts) -> MockTy < 'tcx > { MockTy (PhantomData) } }
    };
}

impl_36!()