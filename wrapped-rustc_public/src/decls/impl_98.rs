macro_rules! deps {
    () => {
        VariantIdx!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: VariantIdx { type T = VariantIdx ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { VariantIdx :: to_val (self . as_usize ()) } }
    };
}

impl_98!()