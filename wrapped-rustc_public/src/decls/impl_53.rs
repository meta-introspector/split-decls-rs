macro_rules! deps {
    () => {
        CrateNum!();
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl RustcInternal for CrateNum { type T < 'tcx > = rustc_span :: def_id :: CrateNum ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { rustc_span :: def_id :: CrateNum :: from_usize (* self) } }
    };
}

impl_53!();