macro_rules! deps {
    () => {
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl RustcInternal for MonoItem { type T < 'tcx > = rustc_middle :: mir :: mono :: MonoItem < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { use rustc_middle :: mir :: mono as rustc_mono ; match self { MonoItem :: Fn (instance) => rustc_mono :: MonoItem :: Fn (instance . internal (tables , tcx)) , MonoItem :: Static (def) => rustc_mono :: MonoItem :: Static (def . internal (tables , tcx)) , MonoItem :: GlobalAsm (_) => { unimplemented ! () } } } }
    };
}

impl_72!();