macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: Local { type T = crate :: mir :: Local ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { self . as_usize () } }
    };
}

impl_144!();