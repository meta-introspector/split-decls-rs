macro_rules! deps {
    () => {
        RustcInternal!();
        VariantIdx!();
        InternalCx!();
        BridgeTys!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl RustcInternal for VariantIdx { type T < 'tcx > = rustc_abi :: VariantIdx ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { rustc_abi :: VariantIdx :: from (self . to_index ()) } }
    };
}

impl_69!();