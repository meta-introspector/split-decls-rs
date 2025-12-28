macro_rules! deps {
    () => {
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl RustcInternal for Place { type T < 'tcx > = rustc_middle :: mir :: Place < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { rustc_middle :: mir :: Place { local : rustc_middle :: mir :: Local :: from_usize (self . local) , projection : tcx . mk_place_elems (& self . projection . internal (tables , tcx)) , } } }
    };
}

impl_90!()