macro_rules! deps {
    () => {
        ExistentialProjection!();
        InternalCx!();
        ExistentialProjectionHelpers!();
        RustcInternal!();
        BridgeTys!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl RustcInternal for ExistentialProjection { type T < 'tcx > = rustc_ty :: ExistentialProjection < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { use crate :: unstable :: internal_cx :: ExistentialProjectionHelpers ; tcx . new_from_args (self . def_id . 0 . internal (tables , tcx) , self . generic_args . internal (tables , tcx) , self . term . internal (tables , tcx) ,) } }
    };
}

impl_79!()