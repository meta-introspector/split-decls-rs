macro_rules! deps {
    () => {
        TyConst!();
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl RustcInternal for TyConst { type T < 'tcx > = InternalConst < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . ty_consts [self . id]) . unwrap () } }
    };
}

impl_59!()