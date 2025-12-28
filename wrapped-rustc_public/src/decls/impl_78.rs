macro_rules! deps {
    () => {
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
        ExistentialPredicate!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl RustcInternal for ExistentialPredicate { type T < 'tcx > = rustc_ty :: ExistentialPredicate < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { ExistentialPredicate :: Trait (trait_ref) => { rustc_ty :: ExistentialPredicate :: Trait (trait_ref . internal (tables , tcx)) } ExistentialPredicate :: Projection (proj) => { rustc_ty :: ExistentialPredicate :: Projection (proj . internal (tables , tcx)) } ExistentialPredicate :: AutoTrait (trait_def) => { rustc_ty :: ExistentialPredicate :: AutoTrait (trait_def . 0 . internal (tables , tcx)) } } } }
    };
}

impl_78!()