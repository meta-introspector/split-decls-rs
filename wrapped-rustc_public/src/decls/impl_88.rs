macro_rules! deps {
    () => {
        RustcInternal!();
        BridgeTys!();
        Span!();
        InternalCx!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl RustcInternal for Span { type T < 'tcx > = rustc_span :: Span ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tables . spans [* self] } }
    };
}

impl_88!()