macro_rules! deps {
    () => {
        Ty!();
        InternalCx!();
        BridgeTys!();
        RustcInternal!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl RustcInternal for Ty { type T < 'tcx > = InternalTy < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . types [* self]) . unwrap () } }
    };
}

impl_58!();