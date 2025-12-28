macro_rules! deps {
    () => {
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
        GenericArgs!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl RustcInternal for GenericArgs { type T < 'tcx > = rustc_ty :: GenericArgsRef < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { InternalCx :: mk_args_from_iter (tcx , self . 0 . iter () . map (| arg | arg . internal (tables , tcx))) } }
    };
}

impl_55!()