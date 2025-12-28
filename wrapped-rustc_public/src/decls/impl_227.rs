macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        Symbol!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_span :: Symbol { type T = crate :: Symbol ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { self . to_string () } }
    };
}

impl_227!();