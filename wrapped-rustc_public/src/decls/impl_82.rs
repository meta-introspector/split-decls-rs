macro_rules! deps {
    () => {
        BridgeTys!();
        InternalCx!();
        TraitRef!();
        RustcInternal!();
        TraitRefHelpers!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl RustcInternal for TraitRef { type T < 'tcx > = rustc_ty :: TraitRef < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { use crate :: unstable :: internal_cx :: TraitRefHelpers ; tcx . new_from_args (self . def_id . 0 . internal (tables , tcx) , self . args () . internal (tables , tcx)) } }
    };
}

impl_82!();