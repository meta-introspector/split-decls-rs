macro_rules! deps {
    () => {
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
        ExistentialTraitRef!();
        ExistentialTraitRefHelpers!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl RustcInternal for ExistentialTraitRef { type T < 'tcx > = rustc_ty :: ExistentialTraitRef < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { use crate :: unstable :: internal_cx :: ExistentialTraitRefHelpers ; tcx . new_from_args (self . def_id . 0 . internal (tables , tcx) , self . generic_args . internal (tables , tcx) ,) } }
    };
}

impl_81!();